import Color from 'colorjs.io'

export const mixColours = (color1: string, color2: string, weight: number): string => {
	const c1 = new Color(color1)
	const c2 = new Color(color2)
	return c1.mix(c2, weight).toString()
}

export const addAlpha = (color: string, alpha: number): string => {
	const c = new Color(color)
	c.alpha = alpha
	return c.toString({ format: 'rgba' })
}
