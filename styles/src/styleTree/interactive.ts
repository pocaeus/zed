import { DeepPartial } from "utility-types";
import merge from "ts-deepmerge"
interface Interactive<T> {
  default: T,
  hover?: T,
  clicked?: T,
  disabled?: T,
}

type InteractiveStates = "default" | "clicked" | "hovered" | "selected" | "disabled";
type InteractiveFields<T> = Record<InteractiveStates, DeepPartial<T>>;
type InteractiveInput<T> = {
  base: T,
  fields: InteractiveFields<T>
}
/**
 * Helper function for creating Interactive<T> objects that works pretty much like Toggle<T>.
 * It takes a object to be used as a value for `default` field and then fills out other fields
 * with fields from either `base` or `modifications`.
 * Notably, it does not touch `hover`, `clicked` and `disabled` if there are no modifications for it.
 *
 * @param defaultObj Object to be used as the value for `default` field.
 * @param base Object containing base fields to be included in the resulting object.
 * @param modifications Object containing modified fields to be included in the resulting object.
 * @returns Interactive<T> object with fields from `base` and `modifications`.
 */
export function interactive<T extends Object>({ base, fields }: InteractiveInput<T>): Interactive<T> {
  let interactiveObj: Interactive<T> = {
    default: base,
  };
  if (fields.default !== undefined) {
    interactiveObj.default = merge(interactiveObj.default, fields.default) as T;
  }
  if (fields.hovered !== undefined) {
    interactiveObj.hover = merge(interactiveObj.default, fields.hovered) as T;
  }

  if (fields.clicked !== undefined) {
    interactiveObj.clicked = merge(interactiveObj.default, fields.clicked) as T;
  }

  if (fields.disabled !== undefined) {
    interactiveObj.disabled = merge(interactiveObj.default, fields.disabled) as T;
  }

  return interactiveObj;
}
