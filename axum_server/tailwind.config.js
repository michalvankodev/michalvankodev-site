/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./templates/**/**.html"],
	theme: {
		extend: {
			fontFamily: {
				sans: [
					"Baloo2",
					"Comfortaa",
					"ui-sans-serif",
					"system-ui",
					"sans-serif",
					"Apple Color Emoji",
					"Segoe UI Emoji",
					"Segoe UI Symbol",
					"Noto Color Emoji",
				],
			},
			spacing: {
				note: "60rem",
				read: "64rem",
				image: "70rem",
				maxindex: "100rem",
			},
			width: {
				note: "60rem",
				read: "64rem",
				image: "70rem",
				maxindex: "100rem",
			},
		},
	},
	plugins: [],
};
