<template>
  <n-card title="Network" id="a">
    <div id="svg" style="width: 100%; height: 500px;"></div>
  </n-card>
</template>

<script lang="ts">
import { Options, Vue } from 'vue-class-component';
import { NCard } from 'naive-ui';
import { drawMinimalSystemInfo, MapNode } from '@/services/map';

const compose = <T>(init: (x: T) => T, ...f: Array<(x: T) => T>) => f.reduceRight((prev, next) => value => prev(next(value)), init);
const pipe = <T>(init: (x: T) => T, ...f: Array<(x: T) => T>) => f.reduce((prev, next) => value => next(prev(value)), init);

const addAttribute  = (key: string, value: string | number) => (e: SVGElement): SVGElement => { e.setAttribute(key, value.toString()); return e };
const addText       = (value: string) => (e: SVGElement): SVGElement => { e.innerHTML = value; return e };
const createElement = (name: string): SVGElement => document.createElementNS('http://www.w3.org/2000/svg', name);

const EXPAND_VIEWBOX_PER_NODE: number = 25;
const BOX_SIZE_X:              number = 150;
const BOX_SIZE_Y:              number = 100;
const INFO_SIZE:               number = 5;

const COLOR_DEFAULT:      string = '#3f3f3f';
const COLOR_DROPOFF:      string = '#f2c97d';
const COLOR_ICE:          string = '#70c0e8';
const COLOR_MERCOXIT:     string = '#e88080';
const COLOR_NON_FRIENDLY: string = '#e88080';
const COLOR_PROC:         string = '#63e2b7';
const COLOR_PORP:         string = '#63e2b7';

const COLOR_OUTER_RECT:  string = '#26262A';
const COLOR_SYSTEM_TEXT: string = '#ffffff';

@Options({
  components: {
    NCard,
  }
})
export default class Network extends Vue {
  /*public nodes: INode[] = [{
    id: 'P-ZMZV',
    connections: ['9CG6-H'],
    cx: 750,
    cy: 150,
    data: {
      nonFriendly: 1,
      system: 'P-ZMZV',
      mercoxit: true,
      ice: true,
      dropoff: true,
      procA: true,
      procB: true,
      porp:  true
    }
  }, {
    id: '9CG6-H',
    connections: ['W6V-VM', 'UYU-VV', 'NDII-Q'],
    cx: 500,
    cy: 150,
    data: {
      nonFriendly: 0,
      system: '9CG6-H',
      mercoxit: false,
      ice: false,
      dropoff: false,
      procA: false,
      procB: false,
      porp:  false
    }
  }, {
    id: 'W6V-VM',
    connections: [],
    cx: 500,
    cy: 0,
    data: {
      nonFriendly: 0,
      system: 'W6V-VM',
      mercoxit: false,
      ice: false,
      dropoff: false,
      procA: false,
      procB: false,
      porp:  false
    }
  }, {
    id: 'UYU-VV',
    connections: [],
    cx: 500,
    cy: 300,
    data: {
      nonFriendly: 0,
      system: 'UYU-VV',
      mercoxit: false,
      ice: false,
      dropoff: false,
      procA: false,
      procB: false,
      porp:  false
    }
  }, {
    id: 'NDII-Q',
    connections: ['K-L690'],
    cx: 250,
    cy: 150,
    data: {
      nonFriendly: 0,
      system: 'NDII-Q',
      mercoxit: false,
      ice: false,
      dropoff: false,
      procA: false,
      procB: false,
      porp:  false
    }
  }, {
    id: 'K-L690',
    connections: [],
    cx: 0,
    cy: 150,
    data: {
      nonFriendly: 0,
      system: 'K-L690',
      mercoxit: false,
      ice: true,
      dropoff: false,
      procA: false,
      procB: false,
      porp:  false
    }
  }];

  public mounted() {
    let a = drawMinimalSystemInfo(
      1500,
      500,
      new MapNode(0, 'P-ZMZV').ice().mercoxit().dropoff()
    );
    console.log(a)

    const calcViewBox = (countNodes: number): number => EXPAND_VIEWBOX_PER_NODE * countNodes;
    const offsetNodeFromBorder = (nodeBorderOffset: number, max: number) => (x: number): number => x >= (max - (2 * nodeBorderOffset))
      ? x -= nodeBorderOffset
      : x <= (2 * nodeBorderOffset)
        ? x += 2 * nodeBorderOffset
        : x;
    const containerHeight = (): number => document.getElementById('svg')?.offsetHeight || 250;
    const containerWidth  = (): number => document.getElementById('svg')?.offsetWidth  || 250;

    const svgElement = (): SVGElement => pipe(
      addAttribute('viewBox', `0 0 ${containerWidth()} ${containerHeight()}`),
      addAttribute('width',  containerWidth()),
      addAttribute('height', containerHeight()),
      addAttribute('preserveAspectRatio', 'xMidYMid meet')
    )(createElement('svg'));

    const generatePosition = (
      id: string,
      connections: string[],
      nodes: INode[],
      data: any,
      cx: number,
      cy: number
    ): INode => {
      return {
        id,
        connections,
        data,
        cx: compose(offsetNodeFromBorder(0, calcViewBox(nodes.length)))(cx),
        cy: compose(offsetNodeFromBorder(0, calcViewBox(nodes.length)))(cy)
      }
    };
    const generateRootNodePosition = (nodes: INode[]): INode[] => nodes
      .map(x => generatePosition(
        x.id,
        x.connections,
        nodes,
        x.data,
        x.cx,
        x.cy
      ));

    const getNodePosition = (id: string, nodes: INode[]): INode => nodes.find(x => x.id === id);
    const generateNodePositions = (nodes: INode[]): INodePosition[] => {
      const root = generateRootNodePosition(nodes);
      return root;
    };

    const calcPath = (source: INode, target: INode): string => {
      let xModifier = BOX_SIZE_X / 2;
      let yModifier = BOX_SIZE_Y / 2;

      let m = `M${source.cx},${source.cy}`;
      let l = `L${target.cx},${target.cy}`;

      if (source.cx > target.cx) {
        m = `M${source.cx},${source.cy + yModifier}`;
        l = `L${target.cx + BOX_SIZE_X},${target.cy + yModifier}`;
      } else if (source.cy > target.cy) {
        m = `M${source.cx + xModifier},${source.cy}`;
        l = `L${target.cx + xModifier},${target.cy + BOX_SIZE_Y}`;
      } else if (target.cy > source.cy) {
        m = `M${source.cx + xModifier},${source.cy + BOX_SIZE_Y}`;
        l = `L${target.cx + xModifier},${target.cy}`;
      }
      return m + l;
    }
    const basePath = (source: INode, target: INode): SVGElement => pipe(
      addAttribute('d', calcPath(source, target)),
      addAttribute('stroke', 'white'),
      addAttribute('fill', 'none')
    )(createElement('path'));
    const path = (e: SVGElement): SVGElement => pipe(addAttribute('stroke-width', '0.3'))(e);
    const drawPaths = (nodePosition: INode[]) => (svg: SVGElement): SVGElement => nodePosition
      .map(x => {
        x
          .connections
          .map(y => basePath(x, getNodePosition(y, nodePosition)))
          .map(y => path(y))
          .map(y => svg.appendChild(y));
      })
      .map(_ => svg)
      .pop();

    // Appends all given svg element `e` to a given svg element `g`
    const addChild = (...e: Array<SVGElement>) => (g: SVGElement): SVGElement =>
      e
        .map(x => g.appendChild(x))
        .map(_ => g)
        .pop();

    const outerRect = (node: INode): SVGElement => pipe(
      addAttribute('id', node.id),
      addAttribute('x', node.cx),
      addAttribute('y', node.cy),
      addAttribute('width',  BOX_SIZE_X),
      addAttribute('height', BOX_SIZE_Y),
      addAttribute('fill', COLOR_OUTER_RECT),
    )(createElement('rect'));
    const systemName = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx + 5),
      addAttribute('y', node.cy + 20),
      addAttribute('fill', COLOR_SYSTEM_TEXT),
      addAttribute('font-size', '20px'),
      addText(node.data.system)
    )(createElement('text'));
    const hasNonFriendly = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx + BOX_SIZE_X - 45),
      addAttribute('y', node.cy + 20),
      addAttribute('fill', node.data.nonFriendly > 0 ? COLOR_NON_FRIENDLY : COLOR_DEFAULT),
      addAttribute('font-size', '20px'),
      addText(node.data.nonFriendly.toString())
    )(createElement('text'));
    const dropoff = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx),
      addAttribute('y', node.cy), // systemName offset + systeName + extra space
      addAttribute('width', BOX_SIZE_X),
      addAttribute('height', 1),
      addAttribute('fill', node.data.dropoff ? COLOR_DROPOFF : COLOR_OUTER_RECT),
    )(createElement('rect'));
    const hasIce = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx + 10),
      addAttribute('y', node.cy + 20 + 20 + 5), // systemName offset + systeName + extra space
      addAttribute('fill', node.data.dropoff ? COLOR_ICE : COLOR_DEFAULT),
      addAttribute('width', INFO_SIZE),
      addAttribute('height', INFO_SIZE),
      addText('Ice')
    )(createElement('text'));
    const hasMercoxit = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx + 10),
      addAttribute('y', node.cy + 20 + 20 + 5 + 20), // systemName offset + systeName + extra space + ice
      addAttribute('fill', node.data.dropoff ? COLOR_MERCOXIT : COLOR_DEFAULT),
      addAttribute('width', INFO_SIZE),
      addAttribute('height', INFO_SIZE),
      addText('Mercoxit')
    )(createElement('text'));
    const hasProcA = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx + BOX_SIZE_X - 45),
      addAttribute('y', node.cy + 20 + 20 + 5),
      addAttribute('fill', node.data.procA ? COLOR_PROC : COLOR_DEFAULT),
      addAttribute('width', INFO_SIZE),
      addAttribute('height', INFO_SIZE),
      addText('ProcA')
    )(createElement('text'));
    const hasProcB = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx + BOX_SIZE_X - 45),
      addAttribute('y', node.cy + 20 + 20 + 5 + 20),
      addAttribute('fill', node.data.procB ? COLOR_PROC : COLOR_DEFAULT),
      addAttribute('width', INFO_SIZE),
      addAttribute('height', INFO_SIZE),
      addText('ProcB')
    )(createElement('text'));
    const hasPorp = (node: INode): SVGElement => pipe(
      addAttribute('x', node.cx + BOX_SIZE_X - 45),
      addAttribute('y', node.cy + 20 + 20 + 5 + 20 + 20),
      addAttribute('fill', node.data.porp ? COLOR_PORP : COLOR_DEFAULT),
      addAttribute('width', INFO_SIZE),
      addAttribute('height', INFO_SIZE),
      addText('Porp')
    )(createElement('text'));

    // Mining Laser Field Enhancement Charge
    // Mining Laser Optimization Charge

    const drawNodes = (nodePosition: INode[]) => (svg: SVGElement): SVGElement =>
      nodePosition
        .map(x =>
          addChild(
            outerRect(x),
            systemName(x),
            dropoff(x),
            hasIce(x),
            hasMercoxit(x),
            hasNonFriendly(x),
            hasProcA(x),
            hasProcB(x),
            hasPorp(x),
            a
          )(createElement('g'))
        )
        .map(x => svg.appendChild(x))
        .map(_ => svg)
        .pop();
    const appendSvg = (svg: SVGElement): SVGElement => {
      document.getElementById('svg')?.appendChild(svg);
      return svg;
    };

    const addClickEvents = (events: IEvent, nodePositions: INode[]) => (svg: SVGElement): SVGElement => {
      nodePositions
        .map(x => {
          if(events.onClick) {
            document.getElementById(x.id)?.addEventListener('click', (_) => events.onClick(x));
          }
          return x;
        });
      return svg;
    }

    const generate = (config: IConfig): SVGElement => {
      const positions: INode[] = generateNodePositions(config.nodes);

      return pipe(
          drawPaths(positions),
          drawNodes(positions),
          appendSvg,
          addClickEvents(config.events, positions)
        )(svgElement(config.nodes))
    };

    const drawSvg = () => document.getElementById('svg')?.appendChild(generate({
      nodes: this.nodes,
      events: {
        onClick: (self) => {
          console.log(self)
        }
      }
    }));
    drawSvg()
  }*/
}

/*interface IConfig {
  events?: IEvent;
  nodes:   INode[];
}

interface IEvent {
  onClick: (self: INode) => void;
}

interface INode {
  connections: string[];
  cx:          number;
  cy:          number;
  data:        INodeData;
  id:          string;
}

interface INodeData {
  system:      string,
  nonFriendly: number;
  mercoxit:    boolean,
  ice:         boolean,
  dropoff:     boolean,
  // Eistonen Kodan Sasen Procurer
  procA:       boolean,
  // ChiefOmncronf Procurer
  procB:       boolean,
  // ChiefOmncron Propoise
  porp:        boolean
}*/
</script>

