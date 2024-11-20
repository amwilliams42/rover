<template>
  <div data-tauri-drag-region class="bg-[#3c3c3c] text-[#cccccc] text-xs flex items-center justify-between h-8">
    <Menubar data-tauri-drag-region class="bg-[#3c3c3c] text-[#cccccc] text-xs flex items-center justify-between h-8 border-0 text-xs">
      <template v-for="menu in menuConfig" :key="menu.trigger">
        <MenubarMenu>
          <MenubarTrigger>{{ menu.trigger }}</MenubarTrigger>
          <MenubarContent>
            <template v-for="(item, index) in menu.items" :key="index">
              <MenubarSeparator v-if="item.separator" />
              <MenubarItem
                v-else
                @click="handleMenuItemClick(item)"
                class="hover:bg-muted"
              >
                {{ item.label }}
                <MenubarShortcut v-if="item.shortcut">{{ item.shortcut }}</MenubarShortcut>
              </MenubarItem>
            </template>
          </MenubarContent>
        </MenubarMenu>
      </template>
    </Menubar>

    <div class="flex">
    <button @click="minimize" class="px-4 py-1 hover:bg-[#505050] focus:outline-none transition duration-150 ease-in-out">
      &#8211;
    </button>
    <button @click="toggleMaximize" class="px-4 py-1 hover:bg-[#505050] focus:outline-none transition duration-150 ease-in-out">
      &#9633;
    </button>
    <button @click="close" class="px-4 py-1 hover:bg-[#e81123] focus:outline-none transition duration-150 ease-in-out">
      &#10005;
    </button>
  </div>
  </div>
</template>

<script lang="ts">
import {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarItem,
  MenubarMenu,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from '@/components/ui/menubar'
import { defineComponent, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { menuConfig } from "./menuConfig"; // Import the menu configuration
import router from '@/router';

const appWindow = getCurrentWindow();

export default defineComponent({
  components: {
    Menubar,
    MenubarCheckboxItem,
    MenubarContent,
    MenubarItem,
    MenubarMenu,
    MenubarRadioGroup,
    MenubarRadioItem,
    MenubarSeparator,
    MenubarShortcut,
    MenubarSub,
    MenubarSubContent,
    MenubarSubTrigger,
    MenubarTrigger,
  },
  methods: {
    switchTheme() {
      const html = document.documentElement;
      if (this.selectedTheme === "dark"){
        html.classList.add("dark");
      } else {
        html.classList.remove("dark")
      }
    },
    minimize() {
      appWindow.minimize();
    },
    async toggleMaximize() {
      appWindow.toggleMaximize();
    },
    close() {
      appWindow.close();
    },
    handleMenuItemClick(item: any) {
      if (item.route) {
        router.push(item.route); // Navigate to the specified route
      } else if (item.onClick) {
        item.onClick(); // Execute custom click handler if defined
      } else if (item.action) {
        console.log(`Executing action: ${item.action}`);
      }
    },
  },
  data() {
    return { 
      selectedTheme: ref("light"),
      menuConfig };
  },
  watch: {
    selectedTheme: {
      immediate: true,
      handler() {
        this.switchTheme();
      },
    },
  },
});
</script>