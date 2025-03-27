import type { Config } from "tailwindcss";
import * as daisyui from "daisyui";

const config: Config = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
};

export default config;
