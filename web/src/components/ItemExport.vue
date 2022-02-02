<template>
  <div>
    <h3>List</h3>
    <n-input type="textarea" v-model:value="list" :rows="7" disabled />
    <n-space style="margin-top: 5px">
      <n-tooltip placement="top" trigger="click">
        <template #trigger>
          <n-button @click="copy_list">Copy</n-button>
        </template>

        <span>Copied to clipboard!</span>
      </n-tooltip>
      <n-tooltip placement="top" trigger="click">
        <template #trigger>
          <n-button @click="copy_ingame">Copy ingame</n-button>
        </template>

        <span>Copied to clipboard!</span>
      </n-tooltip>
    </n-space>

    <h3>CSV</h3>
    <n-input type="textarea" v-model:value="csv" :rows="7" disabled />

    <n-space style="margin-top: 5px">
      <n-tooltip placement="top" trigger="click">
        <template #trigger>
          <n-button @click="copy_csv">Copy</n-button>
        </template>

        <span>Copied to clipboard!</span>
      </n-tooltip>

      <n-button
        @click="download_csv"
      >
        Download
      </n-button>
    </n-space>
  </div>
</template>

<script lang="ts">
import { Options, Vue, prop } from 'vue-class-component';
import { NButton, NInput, NSpace, NTooltip } from 'naive-ui';

class Props {
  csvName = prop({
    type:     String,
    required: true,
  });

  /// String array of the fields to show
  formatCsv = prop({
    type:     Array,
    required: true,
  });

  /// String array of the fields to show
  formatList = prop({
    type:     Array,
    required: true,
  });

  items = prop({
    type:     Array,
    required: true,
  });
}

@Options({
  components: {
    NButton,
    NInput,
    NSpace,
    NTooltip
  }
})
export default class ItemExport extends Vue.with(Props) {
  public list: string = '';
  public csv:  string = '';

  public created() {
    console.log(this.items)
    this.generate();
    this.$watch('items', () => this.generate(), { deep: true });
  }

  public copy_csv() {
    navigator.clipboard.writeText(this.csv);
  }

  public copy_list() {
    navigator.clipboard.writeText(this.list);
  }

  public copy_ingame() {
    let a = this.items
      .map((x: any) => `<url=showinfo:${x['type_id']}>${x['name']}</url>`)
      .join('\n');
    navigator.clipboard.writeText(a);
  }

  public download_csv() {
    const csvFile = new Blob([this.csv], { type: 'text/csv' })
    const downloadLink =  document.createElement('a')

    downloadLink.download = `${this.csvName}.csv`
    downloadLink.href = window.URL.createObjectURL(csvFile)
    downloadLink.style.display = 'none'
    document.body.appendChild(downloadLink)

    downloadLink.click()
  }

  private generate() {
      this.generate_csv(<any[]>this.items);
      this.generate_list(<any[]>this.items);
  }

  private generate_csv(data: any[]) {
    let header = (<string[]>this.formatCsv)
      .map((f: string) => `${f}`)
      .join(';');
    this.csv = `${header}\n`;
    this.csv += data
      .map(x =>
        (<string[]>this.formatCsv)
          .map((f: string) => x[f] === undefined ? '' : x[f])
          .join(';')
      )
      .join('\n');
  }

  private generate_list(data: any[]) {
    this.list = data
      .map(x =>
        (<string[]>this.formatList)
          .map((f: string) => `${x[f]}`)
          .join('\t')
      )
      .join('\n');
  }
}
</script>
