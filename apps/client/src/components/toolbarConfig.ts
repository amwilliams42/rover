interface Tool {
    icon: string; // Icon as HTML (can be custom SVG or from a library)
    onClick: () => void; // Function to execute on click
  }

export const toolbarConfig: Tool[] = [
    {
      icon: '<i class="fas fa-clipboard"></i>',
      onClick: () => console.log("Clipboard tool clicked"),
    },
    {
      icon: '<i class="fas fa-pen"></i>',
      onClick: () => console.log("Pen tool clicked"),
    },
    {
      icon: '<i class="fas fa-map-marker-alt"></i>',
      onClick: () => console.log("Location tool clicked"),
    },
    {
      icon: '<i class="fas fa-cog"></i>',
      onClick: () => console.log("Settings tool clicked"),
    },
  ];
  