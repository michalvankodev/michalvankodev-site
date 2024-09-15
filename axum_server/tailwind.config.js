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
				image: "min(70rem, 95vw)",
				maxindex: "100rem",
			},
			width: {
				note: "60rem",
				read: "64rem",
				image: "min(70rem, 95vw)",
				maxindex: "100rem",
			},
			fontSize: {
				readxl: [
					"1.75rem",
					{
						lineHeight: "2.25rem",
						letterSpacing: "-0.015em",
						fontWeight: "400",
					},
				],
			},
		},
	},
	plugins: [],
};
