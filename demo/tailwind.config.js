module.exports = {
    content: ["./src/**/*.rs", "../daisyui/src/**/*.rs", "../hero_icons/src/**/*.rs" , "index.html"],
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
