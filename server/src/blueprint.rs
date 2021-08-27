use crate::{error::EveServerError, eve::EveAuthService, industry::IndustryService};

use cachem::ConnectionPool;
use caph_db::{Activity, BlueprintEntry, CacheName, IndustryCostEntry, MarketPriceEntry, Material, SchematicEntry};
use caph_eve_data_wrapper::{ItemId, SolarSystemId, TypeId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub struct BlueprintService {
    pool:     ConnectionPool,
    eve_auth: EveAuthService,
    industry: IndustryService,
}

impl BlueprintService {
    pub fn new(
        pool:     ConnectionPool,
        eve_auth: EveAuthService,
        industry: IndustryService,
    ) -> Self {
        Self {
            pool,
            eve_auth,
            industry,
        }
    }

    pub async fn all(
        &self,
    ) -> Result<Vec<Option<BlueprintEntry>>, EveServerError> {
        let mut con = self
            .pool
            .acquire()
            .await?;

        let keys = con
            .keys::<_, TypeId>(CacheName::Blueprint)
            .await?;

        con
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, keys)
            .await
            .map_err(Into::into)
    }

    /// Gets a blueprint by id
    ///
    /// # Params
    ///
    /// * `bpid` -> TypeId of the blueprint to fetch
    ///
    /// # Returns
    ///
    /// Some(BlueprintEntry) if the blueprint exists and None if there is no
    /// blueprint with that id
    ///
    pub async fn by_id(
        &self,
        bpid: TypeId,
    ) -> Result<Option<BlueprintEntry>, EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .get::<_, _, BlueprintEntry>(CacheName::Blueprint, bpid)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of blueprints by id
    ///
    /// # Params
    ///
    /// * `bpids` -> Array of TypeIds
    ///
    /// # Returns
    ///
    /// Array of option containing either Some(BlueprintEntry) or None if there
    /// is no blueprint with that id
    ///
    /// TODO: consider removing option and only return valid
    pub async fn by_ids(
        &self,
        bpids: Vec<TypeId>
    ) -> Result<Vec<Option<BlueprintEntry>>, EveServerError> {
        self
            .pool
            .acquire()
            .await?
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, bpids)
            .await
            .map_err(Into::into)
    }

    /// Calculates the cost of manufacturing for each given blueprint in the
    /// given system
    ///
    /// # Params
    ///
    /// * `bpids` -> Map of blueprint ids and the number of runs
    /// * `sid`   -> Id of the system where
    ///
    /// # Returns
    ///
    /// Vector of [ManufactureCost] contining the cost of all blueprints
    ///
    pub async fn manufacture_cost(
        &self,
        bpids: HashMap<TypeId, u32>,
        sid:   SolarSystemId,
    ) -> Result<Vec<ManufactureCost>, EveServerError> {
        let mut con = self.pool.acquire().await?;

        let bp_keys = bpids
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        let bps = con
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, bp_keys)
            .await?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

        let mut bp_costs = Vec::new();
        for bp in bps {
            let runs = bpids.get(&bp.bid).unwrap_or(&1u32);

            let production = bp.production_activity();

            let product_id   = production.product_id();
            let materials    = production.materials();
            let material_ids = production.material_ids();

            let prices = con
                .mget::<_, _, MarketPriceEntry>(CacheName::MarketPrice, material_ids)
                .await?
                .into_iter()
                .flatten()
                .map(|x| (x.type_id, x))
                .collect::<HashMap<_, _>>();

            let materials = materials
                .into_iter()
                .map(|x| {
                    let price = prices
                        .get(&x.mid)
                        .map(|x| x.adjusted_price)
                        .unwrap_or(0f32);
                    let total = runs * x.quantity;

                    MaterialCost {
                        mid:    x.mid,
                        amount: total,
                        cost:   f32::round(total as f32 * price)
                    }
                })
                .collect::<Vec<_>>();

            // TODO: replace with database -> Requires database and frontend
            let facility_tax_perc = self
                .industry
                .stations()
                .unwrap()
                .iter()
                .find(|x| x.id == *sid)
                .unwrap()
                .engineering
                .manufacturing;

            let material_total_cost: f32 = materials
                .iter()
                .map(|x| x.cost)
                .sum();
            // TODO: consider merging this with the facility cache
            let system_cost_index_perc = con
                .get::<_, _, IndustryCostEntry>(CacheName::IndustryCost, sid)
                .await?
                .unwrap()
                .cost_indices
                .iter()
                .find(|x| x.activity == "manufacturing")
                .unwrap()
                .cost_index;
            // TODO: move to facility cache
            let facility_bonus_perc = 5f32;

            let system_cost_index = f32::round(material_total_cost * system_cost_index_perc);
            let facility_bonus = f32::round(system_cost_index * (facility_bonus_perc / 100f32));

            let mut production_cost = system_cost_index - facility_bonus;
            let facility_tax = f32::round(production_cost * (facility_tax_perc as f32 / 100f32));
            production_cost += facility_tax;

            let sell_price = con
                .get::<_, _, MarketPriceEntry>(CacheName::MarketPrice, product_id)
                .await?
                .unwrap()
                .adjusted_price;
            let sell_price = f32::round(sell_price);
            let total_cost = material_total_cost + production_cost;

            bp_costs.push(ManufactureCost {
                material_total_cost,
                facility_bonus,
                facility_bonus_perc,
                facility_tax,
                facility_tax_perc,
                system_cost_index,
                system_cost_index_perc,
                production_cost,
                total_cost,
                sell_price,
                materials,
                bpid: bp.bid,
            });
        }

        Ok(bp_costs)
    }

    /// Breaks down all higher materials to their base material
    pub async fn tree(
        &self,
        bpids: Vec<TypeId>
    ) -> Result<Vec<BlueprintTreeEntry>, EveServerError> {
        let mut trees = Vec::new();

        for bpid in bpids {
            let base_bp = self
                .pool
                .acquire()
                .await?
                .get::<_, _, BlueprintEntry>(CacheName::Blueprint, bpid)
                .await?
                .ok_or(EveServerError::BlueprintNotFound)?;
            let activity = if let Some(y) = base_bp.manufacture {
                y
            } else if let Some(y) = base_bp.reaction {
                y
            } else {
                return Err(EveServerError::BlueprintNotFound);
            };

            let bps     = self.blueprint_product().await?;
            let schemas = self.schema_blueprint().await?;

            let (product_id, product_quantity) = if let Some(x) = activity.products {
                (x[0].mid, x[0].quantity)
            } else {
                return Err(EveServerError::BlueprintNotFound);
            };

            let materials = self.tree_leaf(&product_id, &bps, &schemas);
            let tree = BlueprintTreeEntry {
                type_id:  product_id,
                quantity: product_quantity,
                leafs:    materials.unwrap_or_default()
            };
            trees.push(tree);
        }

        Ok(trees)
    }

    fn tree_leaf(
        &self,
        product: &TypeId,
        bps:     &HashMap<TypeId, Activity>,
        schemas: &HashMap<TypeId, SchematicEntry>
    ) -> Option<Vec<BlueprintTreeEntry>> {
        if let Some(bp) = bps.get(&product) {
            let material = bp
                .materials
                .clone()
                .unwrap_or_default()
                .into_iter()
                .map(|x| {
                    let res = self.tree_leaf(&x.mid, &bps, &schemas);
                    if let Some(leafs) = res {
                        BlueprintTreeEntry {
                            type_id:  x.mid,
                            quantity: x.quantity,
                            leafs
                        }
                    } else {
                        BlueprintTreeEntry {
                            type_id:  x.mid,
                            quantity: x.quantity,
                            leafs:    Vec::new()
                        }
                    }
                })
                .collect::<Vec<_>>();
            Some(material)
        } else if let Some(schema) = schemas.get(&product) {
            let material = schema
                .inputs
                .clone()
                .into_iter()
                .map(|x| {
                    let res = self.tree_leaf(&x.pid, &bps, &schemas);
                    if let Some(leafs) = res {
                        BlueprintTreeEntry {
                            type_id:  x.pid,
                            quantity: x.quantity,
                            leafs
                        }
                    } else {
                        BlueprintTreeEntry {
                            type_id:  x.pid,
                            quantity: x.quantity,
                            leafs:    Vec::new()
                        }
                    }
                })
                .collect::<Vec<_>>();
            Some(material)
        } else {
            None
        }
    }

    /// Resolves all required materials from a set of blueprints
    pub async fn materials(
        &self,
        bpids: Vec<BlueprintInfo>
    ) -> Result<Vec<Material>, EveServerError> {
        let bps = bpids.iter().map(|x| *x.bpid).collect::<_>();
        let bps = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, bps)
            .await?
            .into_iter()
            .flatten()
            .map(|x| (x.bid, x))
            .collect::<HashMap<_, _>>();
        let mut materials = HashMap::new();

        for bp in bpids {
            let bpid = bp.bpid;

            let base_bp = bps
                .get(&bpid)
                .unwrap();
            let activity = if let Some(y) = &base_bp.manufacture {
                y
            } else if let Some(y) = &base_bp.reaction {
                y
            } else {
                return Err(EveServerError::BlueprintNotFound);
            };

            activity
                .materials
                .as_ref()
                .unwrap_or(&Vec::new())
                .into_iter()
                .for_each(|x| {
                    materials
                        .entry(x.mid)
                        .and_modify(|e: &mut Material| e.quantity += x.quantity)
                        .or_insert(*x);
                });
        }

        let materials = materials
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(materials)
    }

    pub async fn raw_materials(
        &self,
        bpids: Vec<BlueprintInfo>
    ) -> Result<Vec<Material>, EveServerError> {
        let mut materials = HashMap::new();

        let bps = bpids.iter().map(|x| *x.bpid).collect::<_>();
        let bps = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, bps)
            .await?
            .into_iter()
            .flatten()
            .map(|x| (x.bid, x))
            .collect::<HashMap<_, _>>();

        let bp_products = self.blueprint_product().await?;
        let schemas     = self.schema_blueprint().await?;

        for bp in bpids {
            let bpid = bp.bpid;

            let base_bp = bps
                .get(&bpid)
                .unwrap();
            let activity = base_bp.production_activity();

            let mut ids_todo = VecDeque::new();
            if let Some(x) = &activity.products {
                ids_todo.push_front((x[0].mid, x[0].quantity));
            } else {
                return Err(EveServerError::BlueprintNotFound);
            };

            while let Some((c_pid, c_quantity)) = ids_todo.pop_front() {
                if let Some(bp) = bp_products.get(&c_pid) {
                    bp
                        .materials
                        .as_ref()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .for_each(|x| ids_todo.push_back((x.mid, x.quantity)));
                } else if let Some(schema) = schemas.get(&c_pid) {
                    schema
                        .inputs
                        .iter()
                        .for_each(|x| ids_todo.push_back((x.pid, x.quantity)));
                } else {
                    materials
                        .entry(c_pid)
                        .and_modify(|x: &mut Material| x.quantity += c_quantity * bp.runs)
                        .or_insert(Material {
                            mid:         c_pid,
                            quantity:    c_quantity * bp.runs,
                            probability: None
                        });
                }
            }
        }

        let materials = materials
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(materials)
    }

    pub async fn required_blueprints(
        &self,
        bpids: Vec<TypeId>
    ) -> Result<Vec<TypeId>, EveServerError> {
        let mut blueprints = Vec::new();

        let product_bp = self.product_blueprint().await?;
        let mut con = self.pool.acquire().await?;

        for bpid in bpids {
            let base_bp = con
                .get::<_, _, BlueprintEntry>(CacheName::Blueprint, bpid)
                .await?
                .unwrap();
            let activity = base_bp.production_activity();

            let mut ids_todo = VecDeque::new();
            ids_todo.push_front(activity.product_id());

            while let Some(pid) = ids_todo.pop_front() {
                if let Some(bp) = product_bp.get(&pid) {
                    blueprints.push(*bp);

                    let bp = con
                        .get::<_, _, BlueprintEntry>(CacheName::Blueprint, *bp)
                        .await?;
                    if let Some(e) = bp {
                        e
                            .production_activity()
                            .materials
                            .as_ref()
                            .unwrap_or(&Vec::new())
                            .iter()
                            .for_each(|x| ids_todo.push_back(x.mid));
                    }
                }
            }
        }

        blueprints.sort();
        blueprints.dedup();
        Ok(blueprints)
    }

    pub async fn manufacture(
        &self,
        bpids: Vec<BlueprintInfo>
    ) -> Result<Vec<ProductionProduct>, EveServerError> {
        let mut materials = HashMap::new();

        let bps = bpids.iter().map(|x| *x.bpid).collect::<_>();
        let bps = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, bps)
            .await?
            .into_iter()
            .flatten()
            .map(|x| (x.bid, x))
            .collect::<HashMap<_, _>>();

        let bp_products = self.blueprint_product().await?;
        let product_bp  = self.product_blueprint().await?;
        let schemas     = self.schema_blueprint().await?;

        for bp in bpids {
            let bpid = bp.bpid;

            let base_bp = bps
                .get(&bpid)
                .unwrap();
            let activity = base_bp.production_activity();

            let mut ids_todo = VecDeque::new();
            if let Some(x) = &activity.products {
                ids_todo.push_front((x[0].mid, x[0].quantity, 0));
            } else {
                return Err(EveServerError::BlueprintNotFound);
            };

            while let Some((c_pid, c_quantity, depth)) = ids_todo.pop_front() {
                if let Some(bp_product) = bp_products.get(&c_pid) {
                    let bpid = product_bp.get(&c_pid).unwrap();
                    materials
                        .entry(c_pid)
                        .and_modify(|x: &mut ProductionProduct| x.quantity += c_quantity)
                        .or_insert(ProductionProduct {
                            bpid:      *bpid,
                            pid:       c_pid,
                            quantity:  c_quantity * bp.runs,
                            materials: bp_product.materials.clone().unwrap_or_default(),
                            depth,
                        });
                    bp_product
                        .materials
                        .as_ref()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .for_each(|x| ids_todo.push_back((x.mid, x.quantity, depth + 1)));
                } else if let Some(schema) = schemas.get(&c_pid) {
                    schema
                        .inputs
                        .iter()
                        .for_each(|x| ids_todo.push_back((x.pid, x.quantity, depth + 1)));
                }
            }
        }

        let materials = materials
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<_>>();
        Ok(materials)
    }

    /// Loads all blueprints and maps them into a key value format.
    ///
    /// The key represents the product of the blueprint and the value is either
    /// a manufacture activity or a reaction activity
    ///
    async fn blueprint_product(&self) -> Result<HashMap<TypeId, Activity>, EveServerError> {
        let bps = self
            .pool
            .acquire()
            .await?
            .keys::<_, TypeId>(CacheName::Blueprint)
            .await?;
        let bps = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, bps)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .filter(|x| {
                if let Some(man) = x.clone().manufacture {
                    man.products.is_some()
                } else if let Some(reaction) = x.clone().reaction {
                    reaction.products.is_some()
                } else {
                    false
                }
            })
            .map(|x| {
                let activity = x.production_activity();
                (activity.product_id(), activity)
            })
            .collect::<HashMap<_, _>>();
        Ok(bps)
    }

    /// Loads all blueprints and maps them into a key value format `product -> bpid`.
    ///
    /// The key represents the product of the blueprint and the value is the blueprint 
    /// id that produces the item
    ///
    /// TODO: The database should do this using a key value index
    ///
    async fn product_blueprint(&self) -> Result<HashMap<TypeId, TypeId>, EveServerError> {
        let bps = self
            .pool
            .acquire()
            .await?
            .keys::<_, TypeId>(CacheName::Blueprint)
            .await?;
        let bps = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, BlueprintEntry>(CacheName::Blueprint, bps)
            .await?
            .into_iter()
            .flatten()
            .filter(|x| {
                if let Some(man) = x.clone().manufacture {
                    man.products.is_some()
                } else if let Some(reaction) = x.clone().reaction {
                    reaction.products.is_some()
                } else {
                    false
                }
            })
            .map(|x| {
                let activity = x.production_activity();
                (activity.product_id(), x.bid)
            })
            .collect::<HashMap<_, _>>();
        Ok(bps)
    }

    async fn schema_blueprint(&self) -> Result<HashMap<TypeId, SchematicEntry>, EveServerError> {
        let schemas = self
            .pool
            .acquire()
            .await?
            .keys::<_, TypeId>(CacheName::Schematic)
            .await?;
        let schemas = self
            .pool
            .acquire()
            .await?
            .mget::<_, _, SchematicEntry>(CacheName::Schematic, schemas)
            .await?
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .map(|x| (x.output.pid, x))
            .collect::<HashMap<TypeId, SchematicEntry>>();
        Ok(schemas)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct BlueprintCostQuery {
    iid:  ItemId,
    runs: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct ManufactureCost {
    material_total_cost:    f32,
    facility_bonus:         f32,
    facility_bonus_perc:    f32,
    facility_tax:           f32,
    facility_tax_perc:      f32,
    system_cost_index:      f32,
    system_cost_index_perc: f32,
    production_cost:        f32,
    total_cost:             f32,
    sell_price:             f32,
    materials:              Vec<MaterialCost>,
    bpid:                   TypeId,
}

#[derive(Clone, Debug, Serialize)]
pub struct MaterialCost {
    mid:    TypeId,
    amount: u32,
    cost:   f32,
}

#[derive(Debug, Serialize)]
pub struct BlueprintTreeEntry {
    pub type_id:  TypeId,
    pub quantity: u32,
    pub leafs:    Vec<BlueprintTreeEntry>,
}

#[derive(Debug, Deserialize)]
pub struct BlueprintInfo {
    pub bpid: TypeId,
    pub runs: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductionProduct {
    pub pid:       TypeId,
    pub bpid:      TypeId,
    pub quantity:  u32,
    pub materials: Vec<Material>,
    pub depth:     u8,
}
