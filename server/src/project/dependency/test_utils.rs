use caph_connector::TypeId;

use super::Dependency;

pub fn dependency(
    name:       String,
    ptype_id:   TypeId,
    products:   i64,
    components: Vec<Dependency>
) -> Dependency {
    Dependency {
        name:             name,
        ptype_id:         ptype_id,
        category_id:      0.into(),
        group_id:         0.into(),
        products:         products,
        products_base:    1i64,
        products_per_run: 1i64,
        time:             1i64,
        time_per_run:     1i64,
        components:       components,
    }
}

pub fn fuel_block(
    name:    String,
    type_id: TypeId,
) -> Dependency {
    let mut isotope = Dependency {
        name:             "".into(),
        ptype_id:         0.into(),
        category_id:      4.into(),
        group_id:         423.into(),
        products_per_run: 0,
        products:         450i64,
        products_base:    450i64,
        time:             0i64,
        time_per_run:     0i64,
        components:       Vec::new()
    };

    match type_id {
        TypeId(4051) => {
            isotope.name = "Nitrogen Isotopes".into();
            isotope.ptype_id = 17888.into();
        }
        TypeId(4246) => {
            isotope.name = "Hydrogen Isotopes".into();
            isotope.ptype_id = 17889.into();
        }
        TypeId(4247) => {
            isotope.name = "Helium Isotopes".into();
            isotope.ptype_id = 16274.into();
        }
        TypeId(4312) => {
            isotope.name = "Oxygen Isotopes".into();
            isotope.ptype_id = 17887.into();
        }
        _ => unimplemented!()
    }

    let mut dep = Dependency {
        name:             name,
        ptype_id:         type_id,
        category_id:      4.into(),
        group_id:         1136.into(),
        products:         0i64,
        products_base:    0i64,
        products_per_run: 40,
        time:             900i64,
        time_per_run:     900i64,
        components:       vec![
            Dependency {
                name:             "Enriched Uranium".into(),
                ptype_id:         44.into(),
                category_id:      43.into(),
                group_id:         1034.into(),
                products_per_run: 0,
                products:         4i64,
                products_base:    4i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            Dependency {
                name:             "Oxygen".into(),
                ptype_id:         3683.into(),
                category_id:      43.into(),
                group_id:         1042.into(),
                products_per_run: 0,
                products:         22i64,
                products_base:    22i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            Dependency {
                name:             "Mechanical Parts".into(),
                ptype_id:         3689.into(),
                category_id:      43.into(),
                group_id:         1034.into(),
                products_per_run: 0,
                products:         4i64,
                products_base:    4i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            Dependency {
                name:             "Coolant".into(),
                ptype_id:         9832.into(),
                category_id:      43.into(),
                group_id:         1034.into(),
                products_per_run: 0,
                products:         9i64,
                products_base:    9i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            Dependency {
                name:             "Robotics".into(),
                ptype_id:         9848.into(),
                category_id:      43.into(),
                group_id:         1040.into(),
                products_per_run: 0,
                products:         1i64,
                products_base:    1i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            Dependency {
                name:             "Heavy Water".into(),
                ptype_id:         16272.into(),
                category_id:      4.into(),
                group_id:         423.into(),
                products_per_run: 0,
                products:         170i64,
                products_base:    170i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            Dependency {
                name:             "Liquid Ozone".into(),
                ptype_id:         16273.into(),
                category_id:      4.into(),
                group_id:         423.into(),
                products_per_run: 0,
                products:         350i64,
                products_base:    350i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            Dependency {
                name:             "Strontium Clathrates".into(),
                ptype_id:         16275.into(),
                category_id:      4.into(),
                group_id:         423.into(),
                products_per_run: 0,
                products:         20i64,
                products_base:    20i64,
                time:             0i64,
                time_per_run:     0i64,
                components:       Vec::new()
            },
            isotope
        ]
    };

    dep.components.sort_by_key(|x| x.ptype_id);
    dep
}
