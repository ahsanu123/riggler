
export enum ActiveTab {
  MainScreen,
  Setting,
  About
}

export interface RigglerAppState {
  activeTab: ActiveTab,
  jigglingDuration: number,
  jigglingDelta: number,
  isJiggling: boolean,
}

export const rigglerAppState: RigglerAppState = {
  activeTab: ActiveTab.MainScreen,
  jigglingDuration: 1,
  jigglingDelta: 1,
  isJiggling: false,
}
