// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    // (optional) Enable the Nuxt devtools
    devtools: { enabled: true },

    // Enable SSG
    ssr: false,

    telemetry: false,

    vite: {
        // Better support for Tauri CLI output
        clearScreen: false,
        // Enable environment variables
        // Additional environment variables can be found at
        // https://tauri.app/2/reference/environment-variables/
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            // Tauri requires a consistent port
            strictPort: true,
            hmr: {
                // Use websocket for mobile hot reloading
                protocol: 'ws',
                // Make sure it's available on the network
                host: '0.0.0.0',
                // Use a specific port for hmr
                port: 5183,
            },
            watch: {
                ignored: [
                    '**/src-tauri/**/*',
                ]
            },
        },
    },

    modules: ["@nuxtjs/tailwindcss", "shadcn-nuxt"],
    shadcn: {
        /**
         * Prefix for all the imported component
         */
        prefix: '',
        /**
         * Directory that the component lives in.
         * @default "./components/ui"
         */
        componentDir: './components/ui',
    },

    ignore: [
        '**/src-tauri/**/*',
    ],
});
