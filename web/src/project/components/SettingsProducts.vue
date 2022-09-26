<template>
  <n-table striped>
    <thead>
      <tr>
        <th>Product name</th>
        <th width="200px">Count</th>
        <th width="200px">Material Efficiency</th>
        <th width="100px"></th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="(product, index) in project.products">
        <td>{{ product.name }}</td>
        <td>{{ product.count }}</td>
        <td>{{ product.meff }}</td>
        <td>
          <n-button
            @click="delete_product"
            style="width: 90px"
            type="error"
            ghost
          >
            Delete
          </n-button>
        </td>
      </tr>
    </tbody>
    <tfoot>
      <tr>
        <td>
          <n-select
            :options="buildable_items"
            @update:value="handle_product_select"
            v-model:value="product.type_id"
            placeholder="Select Item"
            filterable
          />
        </td>
        <td>
          <n-input-number
            v-model:value="product.count"
          />
        </td>
        <td>
          <n-input-number
            v-model:value="product.meff"
          />
        </td>
        <td>
          <n-button
            @click="add_product"
            :disabled="
              !product.name ||
              !product.type_id
            "
            style="width: 90px"
            type="info"
            ghost
          >
            Add
          </n-button>
        </td>
      </tr>
    </tfoot>
  </n-table>
</template>

<script lang="ts">
import { Options, prop, Vue } from 'vue-class-component';
import { NButton, NCard, NInputNumber, NSelect, NTable, SelectOption } from 'naive-ui';
import { IProject, IProduct } from '@/project/service';
import { ItemService } from '@/services/item';

class Props {
  project = prop({
    type:     Object,
    required: true
  });
}

@Options({
  components: {
    NButton,
    NCard,
    NInputNumber,
    NSelect,
    NTable
  }
})
export default class SettingGeneral extends Vue.with(Props) {
  public buildable_items: SelectOption[] = [];

  public project: IProject = <IProject>{  };
  public product: IProduct = this.default_product();

  public async created() {
    // TODO: replace with caching version
    this.buildable_items = (await ItemService.buildable_items()).map(x => {
      return {
        label: x.name,
        value: x.type_id
      }
    });
  }

  // Adds the current set values 
  public add_product() {
    this.project.products.push(this.product);
    this.product = this.default_product();
  }

  // Removes the product on in the array
  public delete_product(index: number) {
    console.log(index)
    this.project.products.splice(index - 1, 1);
    this.product = this.default_product();
  }

  // Sets the name in order to be rendered in the table
  public handle_product_select(_: string, options: SelectOption) {
    this.product.name = <string>options.label;
  }

  public default_product(): IProduct {
    return {
      name:    '',
      type_id: <any>undefined,
      count:   1,
      meff:    0,
    };
  }
}
</script>
