import { join } from "path";
import type { Config } from "tailwindcss";
import typography from "@tailwindcss/typography";
import { skeleton } from "@skeletonlabs/tw-plugin";
import forms from "@tailwindcss/forms";
import tailwindcss3d from "tailwindcss-3d";

export default {
    darkMode: "class",
    content: [
        "./src/**/*.{html,js,svelte,ts}",
        join(
            require.resolve("@skeletonlabs/skeleton"),
            "../**/*.{html,js,svelte,ts}",
        ),
    ],
    theme: {
        extend: {},
    },
    plugins: [
        typography,
        skeleton({
            themes: {
                preset: [
                    {
                        name: "skeleton",
                        enhancements: true,
                    },
                ],
            },
        }),
        forms,
        tailwindcss3d,
    ],
} satisfies Config;
