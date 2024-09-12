/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ["./templates/**/**.html"],
	theme: {
		extend: {
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
