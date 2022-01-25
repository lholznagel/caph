export type CharacterId = number;
export type GroupId     = number;
export type ItemId      = number;
export type LocationId  = number;
export type SystemId    = number;
export type TypeId      = number;

export type Uuid        = string;
export type BudgetId    = Uuid;

export enum ItemGroup {
  All        = 0,
  Minerals   = 18,
  Ice        = 423,
  Moon       = 427,
  Gas        = 711,
  Salvage    = 754,
  PI0Solid   = 1032,
  PI0Liquid  = 1033,
  PI0Organic = 1035,
  PI1        = 1042,
  PI2        = 1034,
  PI3        = 1040,
  PI4        = 1041,
}
