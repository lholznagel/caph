<template>
  <n-text>{{ name || id }}</n-text>
</template>

<script lang="ts">
import { NText } from 'naive-ui';
import { Options, Vue, prop } from 'vue-class-component';

class Props {
  id = prop({
    type:     Number,
    required: true
  })
}

@Options({
  components: {
    NText
  }
})
export default class CategoryName extends Vue.with(Props) {
  public name: string = '';

  public async created() {
    switch (this.id) {
      case 4:
        this.name = 'Material'
        break;
      case 6:
        this.name = 'Ship'
        break;
      case 7:
        this.name = 'Module'
        break;
      case 8:
        this.name = 'Charge'
        break;
      case 9:
        this.name = 'Blueprint'
        break;
      case 18:
        this.name = 'Drone'
        break;
      default:
        this.name = 'Unknown ' + this.id
    }
  }
}
</script>
