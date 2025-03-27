// frontend/src/types/window.d.ts
export {};

declare global {
  interface Window {
    __TAURI__?: boolean;
  }
}
