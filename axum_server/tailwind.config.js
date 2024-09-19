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
			colors: {
				// blue: {
				// 	50: "#ecf6fe",
				// 	100: "#d9edfc",
				// 	200: "#b3dbf9",
				// 	300: "#8ecaf6",
				// 	400: "#68b8f3",
				// 	500: "#42a6f0",
				// 	600: "#3585c0",
				// 	700: "#286490",
				// 	800: "#1F4E71",
				// 	900: "#173A54",
				// 	950: "#0F2637",
				// },
				blue: {
					50: "#f1f7fe",
					100: "#e1effd",
					200: "#bddefa",
					300: "#82c3f7",
					400: "#42a6f0",
					500: "#1789e0",
					600: "#0a6cbf",
					700: "#0a569a",
					800: "#0c4980",
					900: "#103e6a",
					950: "#0b2746",
				},
				// pink: {
				// 	50: "#FFFBFE",
				// 	100: "#FFE4F9",
				// 	200: "#FECEF4",
				// 	300: "#FEB8EF",
				// 	400: "#fea6eb",
				// 	500: "#D38AC3",
				// 	600: "#B476A7",
				// 	700: "#96628B",
				// 	800: "#774E6E",
				// 	900: "#593A52",
				// 	950: "#3A2636",
				// },
				pink: {
					50: "#fff4fd",
					100: "#ffe7fb",
					200: "#ffcff7",
					300: "#fea6eb",
					400: "#fc76dd",
					500: "#f342ca",
					600: "#d722a9",
					700: "#b31889",
					800: "#92166e",
					900: "#771859",
					950: "#500238",
				},
				purple: {
					50: "#F8F5FC",
					100: "#D5C2ED",
					200: "#B28EDE",
					300: "#8F5BCF",
					400: "#6D30B9",
					500: "#5F2AA2",
					600: "#52248A",
					700: "#441E73",
					800: "#36185C",
					900: "#281244",
					950: "#1A0C2D",
				},
			},
		},
	},
	plugins: [],
};
