/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./templates/**/*.html"],
	theme: {
		extend: {
			spacing: {
				note: "60rem",
				read: "64rem",
				image: "70rem",
			},
			width: {
				note: "60rem",
				read: "64rem",
				image: "70rem",
			},
		},
	},
	plugins: [],
};
