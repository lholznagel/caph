const COLOR_DEFAULT:      string = '#3f3f3f';
const COLOR_DROPOFF:      string = '#f2c97d';
const COLOR_ICE:          string = '#70c0e8';
const COLOR_MERCOXIT:     string = '#e88080';
const COLOR_NON_FRIENDLY: string = '#e88080';
const COLOR_OUTER_RECT:   string = '#26262A';
const COLOR_PORP:         string = '#63e2b7';
const COLOR_PROC:         string = '#63e2b7';
const COLOR_SYSTEM_TEXT:  string = '#ffffff';

const compose = <T>(init: (x: T) => T, ...f: Array<(x: T) => T>) => f.reduceRight((prev, next) => value => prev(next(value)), init);
const pipe = <T>(init: (x: T) => T, ...f: Array<(x: T) => T>) => f.reduce((prev, next) => value => next(prev(value)), init);
// Creates a new svg element
const createElement = (name: string): SVGElement => document.createElementNS('http://www.w3.org/2000/svg', name);

// Adds an attribute to the given svg element and returns an updated version
const addAttribute  = (key: string, value: string | number) => (e: SVGElement): SVGElement => { e.setAttribute(key, value.toString()); return e };
// Appends all given svg element `e` to a given svg element `g`
const addChild = (...e: Array<SVGElement>) => (g: SVGElement): SVGElement =>
  e
    .map(x => g.appendChild(x))
    .map(_ => g)
    .pop()!;
const addText = (value: string) => (e: SVGElement): SVGElement => { e.innerHTML = value; return e };

const outerRect = (
  x: number,
  y: number,
  width: number,
  height: number,
  node: MapNode
): SVGElement => pipe(
  addAttribute('id', node.id),
  addAttribute('x', x),
  addAttribute('y', y),
  addAttribute('width',  width),
  addAttribute('height', height),
  addAttribute('fill', COLOR_OUTER_RECT),
)(createElement('rect'));
const systemName = (
  x: number,
  y: number,
  _width: number,
  _height: number,
  node: MapNode
): SVGElement => pipe(
  addAttribute('x', x + 5),
  addAttribute('y', y + 20),
  addAttribute('fill', COLOR_SYSTEM_TEXT),
  addAttribute('font-size', '20px'),
  addText(node.name)
)(createElement('text'));
const nonFriendly = (
  x: number,
  y: number,
  width: number,
  _height: number,
  node: MapNode
): SVGElement => pipe(
  addAttribute('x', x + width - 50),
  addAttribute('y', y + 20),
  addAttribute('fill', node.nonFriendly > 0 ? COLOR_NON_FRIENDLY : COLOR_DEFAULT),
  addAttribute('font-size', '20px'),
  addText(node.nonFriendly.toString())
)(createElement('text'));
const dropoff = (
  x: number,
  y: number,
  width: number,
  _height: number,
  node: MapNode
): SVGElement => pipe(
  addAttribute('x', x),
  addAttribute('y', y),
  addAttribute('width', width),
  addAttribute('height', 1),
  addAttribute('fill', node.isDropoff ? COLOR_DROPOFF : COLOR_OUTER_RECT),
)(createElement('rect'));

export const drawMinimalSystemInfo = (
  x: number,
  y: number,
  ...nodes: Array<MapNode>
): SVGElement => {
  const BOX_SIZE_X: number = 150;
  const BOX_SIZE_Y: number = 25;

  const startX: number = (x - (nodes.length * BOX_SIZE_X)) / 2;
  const startY: number = 0;

  return addChild(
    outerRect(startX, startY, BOX_SIZE_X, BOX_SIZE_Y, nodes[0]),
    systemName(startX, startY, BOX_SIZE_X, BOX_SIZE_Y, nodes[0]),
    nonFriendly(startX, startY, BOX_SIZE_X, BOX_SIZE_Y, nodes[0]),
    dropoff(startX, startY, BOX_SIZE_X, BOX_SIZE_Y, nodes[0]),
  )(createElement('g'));
};

export class MapNode {
  private _id:   number;
  private _name: string;

  private connections: number[] = [];

  private _dropoff:  boolean = false;
  private _ice:      boolean = false;
  private _mercoxit: boolean = false;

  private _nonFriendly: number = 0;

  constructor(
    id:   number,
    name: string,
  ) {
    this._id   = id;
    this._name = name;
  }

  get id(): number {
    return this._id;
  }

  get name(): string {
    return this._name;
  }

  get isDropoff(): boolean {
    return this._dropoff;
  }

  get hasIce(): boolean {
    return this._ice;
  }

  get hasMercoxit(): boolean {
    return this._mercoxit;
  }

  get nonFriendly(): number {
    return this._nonFriendly;
  }

  public addConnection(id: number): MapNode {
    this.connections.push(id);
    return this;
  }

  public dropoff(): MapNode {
    this._dropoff = true;
    return this;
  }

  public ice(): MapNode {
    this._ice = true;
    return this;
  }

  public mercoxit(): MapNode {
    this._mercoxit = true;
    return this;
  }

  public reds(reds: number): MapNode {
    this._nonFriendly = reds;
    return this;
  }
}

