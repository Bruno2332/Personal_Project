import type { ComponentPropsWithoutRef } from "react"

type ButtonProps = ComponentPropsWithoutRef<"button">


export function Button({ type = "button", ...props }: ButtonProps) {
	return <button type={type} {...props} />
}
