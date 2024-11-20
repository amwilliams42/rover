// src/menuConfig.ts

interface MenuItem {
    label: string;
    action?: string;
    route?: string; // Optional route to navigate to
    shortcut?: string;
    separator?: boolean;
    onClick?: () => void;
    items?: MenuItem[]; // Nested menu items for submenus
  }
  
  interface Menu {
    trigger: string;
    items: MenuItem[];
  }
  
  export const menuConfig: Menu[] = [
    {
        trigger: "File",
        items: [

        ]
    },
    {
      trigger: "View",
      items: [
        { label: "Dashboard", route: "/", shortcut: "⌘D" },
        { label: "Settings", route: "/settings", shortcut: "⌘S" },
        { label: "Reports", route: "/reports", shortcut: "⌘R" },
      ],
    },
    {
      trigger: "Administration",
      items: [
        { label: "Zoom In", action: "zoomIn" },
        { label: "Zoom Out", action: "zoomOut" },
      ],
    },
  ];
  