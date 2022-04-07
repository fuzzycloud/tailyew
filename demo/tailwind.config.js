module.exports = {
    content: ["./src/**/*.rs", "index.html"],
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
