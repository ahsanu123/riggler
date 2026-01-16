import { defaultWindowIcon } from "@tauri-apps/api/app";
import { Menu, MenuItem } from "@tauri-apps/api/menu";
import { TrayIcon, TrayIconOptions } from "@tauri-apps/api/tray";

enum TrayMenuId {
  ToggleJiggling = "toogle-jiggling",
  StartJiggling = 'start-jiggling',
  StopJiggling = 'stop-jiggling',
  Exit = 'exit'
}

const defaultTrayMenuItems: TrayMenuId[] = [
  TrayMenuId.ToggleJiggling,
  TrayMenuId.StartJiggling,
  TrayMenuId.StopJiggling,
  TrayMenuId.Exit
]

const handleOnTrayMenuItemClick = (_id: string) => {
  // TODO:
}


export let TRAY_ICON: TrayIcon | undefined = undefined;

export async function createTrayIcon() {

  const menuItems = await Promise.all(
    defaultTrayMenuItems.map(async (menu) =>
      await MenuItem.new({
        id: menu,
        text: menu,
        action: handleOnTrayMenuItemClick,
      })
    )
  )

  const windowIcon = await defaultWindowIcon();

  const menu = await Menu.new({
    items: menuItems
  })

  let trayIconOption: TrayIconOptions = {
    menu,
  }

  if (windowIcon) trayIconOption.icon = windowIcon
  TRAY_ICON = await TrayIcon.new(trayIconOption)
}
