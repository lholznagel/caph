import axios from 'axios';
import { StructureId, TypeId } from '@/utils';
import { SelectGroupOption, SelectOption } from 'naive-ui';

const ROUTE: string = '/api/v2/structures';

export class Service {
  public static async all(): Promise<IStructure[]> {
    //return (await axios.get(`${ROUTE}`)).data;
    return [];
  }

  public static async remove(
    sid: StructureId,
  ): Promise<void> {
    return (await axios.delete(`${ROUTE}/${sid}`));
  }

  public static structure_types(): SelectGroupOption[] {
    return STRUCTURE_TYPES;
  }

  public static rigs(sid: TypeId): SelectGroupOption[] {
    switch (sid) {
      // Raitaru, Astrahus
      case 35825:
      case 35832:
        return [{
          type:     'group',
          label:    'Material Efficiency Engineering',
          key:      'material_efficiency_engineering',
          children: MEDIUM_ENGINEERING_RIGS_MATERIAL
        }, {
          type:     'group',
          label:    'Time Efficiency Engineering',
          key:      'time_efficiency_engineering',
          children: MEDIUM_ENGINEERING_RIGS_TIME
        }, {
          type:     'group',
          label:    'Research',
          key:      'research',
          children: MEDIUM_ENGINEERING_RIGS_RESEARCH
        }];
      // Azbel, Fortizar
      case 35826:
      case 35833:
        return [{
          type:     'group',
          label:    'Manufacturing',
          key:      'manufacturing',
          children: LARGE_ENGINEERING_RIGS_MANUFACTURING
        }, {
          type:     'group',
          label:    'Research',
          key:      'research',
          children: LARGE_ENGINEERING_RIGS_RESEARCH
        }];
      // Sotiyo, Keepstar
      case 35827:
      case 35834:
        return [{
          type:     'group',
          label:    'Manufacturing',
          key:      'manufacturing',
          children: XLARGE_ENGINEERING_RIGS
        }];
      // Athanor
      case 35835:
        return [{
          type:     'group',
          label:    'Material Efficiency Resource',
          key:      'material_efficiency_resource',
          children: MEDIUM_RESOURCE_PROCESSING_RIGS_MATERIAL
        }, {
          type:     'group',
          label:    'Material Efficiency Engineering',
          key:      'material_efficiency_engineering',
          children: MEDIUM_ENGINEERING_RIGS_MATERIAL
        }, {
          type:     'group',
          label:    'Time Efficiency Resource',
          key:      'time_efficiency_resource_resource',
          children: MEDIUM_RESOURCE_PROCESSING_RIGS_TIME
        }, {
          type:     'group',
          label:    'Time Efficiency Engineering',
          key:      'time_efficiency_engineering',
          children: MEDIUM_ENGINEERING_RIGS_TIME
        }, {
          type:     'group',
          label:    'Research',
          key:      'research',
          children: MEDIUM_ENGINEERING_RIGS_RESEARCH
        }];
      // Tatara
      case 35836:
        return [{
          type:     'group',
          label:    'Reactor Efficiency',
          key:      'reactor_efficiency',
          children: LARGE_RESOURCE_PROCESSING_RIGS
        }]
      default:
        return [];
    }
  }
}

export interface IStructure {
  name:     string;
  location: string;
  rigs:     IStructureRig[];
}

export interface IStructureRig {
  value:    number;
  value_t1: number;
  value_t2: number;
}

const STRUCTURE_TYPES: SelectGroupOption[] = [{
  type:  'group',
  label: 'Engineering Complex',
  key:   'engineering_complex',
  children: [{
    label: 'Raitaru',
    value: 35825
  }, {
    label: 'Azbel',
    value: 35826
  }, {
    label: 'Sotiyo',
    value: 35827
  }]
}, {
  type:  'group',
  label: 'Refineries',
  key:   'refineries',
  children: [{
    label: 'Athanor',
    value: 35835
  }, {
    label: 'Tatara',
    value: 35836
  }]
}, {
  type:  'group',
  label: 'Citadels',
  key:   'citadels',
  children: [{
    label: 'Astrahus',
    value: 35832
  }, {
    label: 'Fortizar',
    value: 35833
  }, {
    label: 'Keepstar',
    value: 35834
  }, ]
}, {
  type:  'group',
  label: 'Faction Citadels',
  key:   'faction_citadels',
  children: [{
    label: '\'Draccous\' Fortizar',
    value: 47513
  }, {
    label: '\'Horizon\' Fortizar',
    value: 47514
  }, {
    label: '\'Marginis\' Fortizar',
    value: 47515
  }, {
    label: '\'Moreau\' Fortizar',
    value: 47512
  }, {
    label: '\'Prometheus\' Fortizar',
    value: 47516
  }, {
    label: 'Upwell Palatine Keepstar',
    value: 40340
  }]
}];

const MEDIUM_RESOURCE_PROCESSING_RIGS_MATERIAL: SelectOption[] = [{
  label:   'Biochemical Reactor',
  value:    46494,
  value_t2: 46495
}, {
  label:    'Composite Reactor',
  value:    46486,
  value_t2: 46487,
}, {
  label:    'Hybrid Reactor',
  value:    46490,
  value_t2: 46491
}];

const MEDIUM_RESOURCE_PROCESSING_RIGS_TIME: SelectOption[] = [{
  label:    'Biochemical Reactor',
  value:    46492,
  value_t2: 46493
}, {
  label:    'Composite Reactor',
  value:    46484,
  value_t2: 46485,
}, {
  label:    'Hybrid Reactor',
  value:    46488,
  value_t2: 46489,
}];

const LARGE_RESOURCE_PROCESSING_RIGS: SelectOption[] = [{
  label:    'Reactor Efficiency',
  value:    46496,
  value_t2: 46497
}];

const MEDIUM_ENGINEERING_RIGS_MATERIAL: SelectOption[] = [{
  label:    'Advanced Component Manufacturing',
  value:    43867,
  value_t2: 43866
}, {
  label:    'Advanced Large Ship Manufacturing',
  value:    43862,
  value_t2: 43863
}, {
  label:    'Advanced Medium Ship Manufacturing',
  value:    43858,
  value_t2: 43859
}, {
  label:    'Advanced Small Ship Manufacturing',
  value:    43855,
  value_t2: 43854
}, {
  label:    'Ammunition Manufacturing',
  value:    37158,
  value_t2: 37159
}, {
  label:    'Basic Capital Component Manufacturing',
  value:    43870,
  value_t2: 43871
}, {
  label:    'Basic Large Ship Manufacturing',
  value:    43732,
  value_t2: 37152
}, {
  label:    'Basic Medium Ship Manufacturing',
  value:    37146,
  value_t2: 37147
}, {
  label:    'Basic Small Ship Manufacturing',
  value:    37154,
  value_t2: 37155
}, {
  label:    'Drone and Fighter Manufacturing',
  value:    37156,
  value_t2: 37157
}, {
  label:    'Equipment Manufacturing',
  value:    43920,
  value_t2: 43921
}, {
  label:    'Structure Manufacturing',
  value:    43875,
  value_t2: 43874
}, {
  label:    'Thukker Advanced Component Manufacturing',
  value:    45640,
  value_t2: 45544
}];

const MEDIUM_ENGINEERING_RIGS_TIME: SelectOption[] = [{
  label:    'Advanced Component Manufacturing',
  value:    43869,
  value_t2: 43868
}, {
  label:    'Advanced Large Ship Manufacturing',
  value:    43865,
  value_t2: 43864
}, {
  label:    'Advanced Medium Ship Manufacturing',
  value:    43860,
  value_t2: 43861
}, {
  label:    'Advanced Small Ship Manufacturing',
  value:    43856,
  value_t2: 43857
}, {
  label:    'Ammunition Manufacturing',
  value:    37150,
  value_t2: 37151
}, {
  label:    'Basic Capital Component Manufacturing',
  value:    43872,
  value_t2: 43873
}, {
  label:    'Basic Large Ship Manufacturing',
  value:    43733,
  value_t2: 43734
}, {
  label:    'Basic Medium Ship Manufacturing',
  value:    43919,
  value_t2: 37153
}, {
  label:    'Basic Small Ship Manufacturing',
  value:    37162,
  value_t2: 37163
}, {
  label:    'Drone and Fighter Manufacturing',
  value:    37148,
  value_t2: 37149
}, {
  label:    'Equipment Manufacturing',
  value:    37160,
  value_t2: 37161
}, {
  label:    'Structure Manufacturing',
  value:    43876,
  value_t2: 43877
}];

const MEDIUM_ENGINEERING_RIGS_RESEARCH: SelectOption[] = [{
  label:    'Blueprint Copy Accelerator',
  value:    43893,
  value_t2: 43892
}, {
  label:    'Blueprint Copy Cost Optimization',
  value:    43891,
  value_t2: 43890
}, {
  label:    'Invention Accelerator',
  value:    43880,
  value_t2: 43881
}, {
  label:    'Invention Cost Optimization',
  value:    43879,
  value_t2: 43878
}, {
  label:    'ME Research Accelerator',
  value:    43883,
  value_t2: 43882
}, {
  label:    'ME Research Cost Optimization',
  value:    43885,
  value_t2: 43884
}, {
  label:    'TE Research Accelerator',
  value:    43889,
  value_t2: 43888
}, {
  label:    'TE Research Cost Optimization',
  value:    43887,
  value_t2: 43886
}];

const LARGE_ENGINEERING_RIGS_MANUFACTURING: SelectOption[] = [{
  label:    'Advanced Component Manufacturing',
  value:    37174,
  value_t2: 37175
}, {
  label:    'Advanced Large Ship Manufacturing',
  value:    37168,
  value_t2: 37169
}, {
  label:    'Advanced Medium Ship Manufacturing',
  value:    43709,
  value_t2: 43711
}, {
  label:    'Advanced Small Ship Manufacturing',
  value:    43707,
  value_t2: 43708
}, {
  label:    'Ammunition Manufacturing',
  value:    37164,
  value_t2: 37165
}, {
  label:    'Basic Capital Component Manufacturing',
  value:    43718,
  value_t2: 43719
}, {
  label:    'Basic Large Ship Manufacturing',
  value:    37166,
  value_t2: 37167
}, {
  label:    'Basic Medium Ship Manufacturing',
  value:    43716,
  value_t2: 43717
}, {
  label:    'Basic Small Ship Manufacturing',
  value:    43714,
  value_t2: 43715
}, {
  label:    'Capital Ship Manufacturing',
  value:    37173,
  value_t2: 37172
}, {
  label:    'Drone and Fighter Manufacturing',
  value:    43712,
  value_t2: 43713
}, {
  label:    'Equipment Manufacturing',
  value:    37170,
  value_t2: 37171
}, {
  label:    'Structure Manufacturing',
  value:    43720,
  value_t2: 43721
}, {
  label:    'Thukker Advanced Component Manufacturing',
  value:    45641,
  value_t2: 45546
}];

const LARGE_ENGINEERING_RIGS_RESEARCH: SelectOption[] = [{
  label:    'Blueprint Copy Optimization',
  value:    43729,
  value_t2: 43730
}, {
  label:    'Invention Optimization',
  value:    43722,
  value_t2: 43723
}, {
  label:    'ME Research Optimization',
  value:    43724,
  value_t2: 43725
}, {
  label:    'TE Research Optimization',
  value:    43726,
  value_t2: 43727
}];

const XLARGE_ENGINEERING_RIGS: SelectOption[] = [{
  label:    'Equipment and Consumable Manufacturing',
  value:    37178,
  value_t2: 37179
}, {
  label:    'Laboratory Optimization',
  value:    37183,
  value_t2: 37182
}, {
  label:    'Ship Manufacturing',
  value:    37180,
  value_t2: 37181
}, {
  label:    'Structure and Component Manufacturing',
  value:    43704,
  value_t2: 43705
}, {
  label:    'Thukker Structure and Component Manufacturing',
  value:    45548,
  value_t2: 45548
}]
