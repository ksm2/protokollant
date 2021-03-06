/**
 * Created on 2019-03-28.
 *
 * @author Konstantin Simon Maria Möllers
 */
export class Release {
  [key: string]: any

  private _name: string
  private _description: string = ''
  private _link: string | null = null
  private _date: string | null = null
  private readonly _items = new Map<string, string[]>()

  constructor(name: string) {
    this._name = name
  }

  getName(): string {
    return this._name
  }

  setName(value: string): this {
    this._name = value
    return this
  }

  getDescription(): string {
    return this._description
  }

  setDescription(value: string): this {
    this._description = value
    return this
  }

  getLink(): string | null {
    return this._link
  }

  setLink(value: string | null): this {
    this._link = value
    return this
  }

  getDate(): string | null {
    return this._date
  }

  setDate(value: string | null): this {
    this._date = value
    return this
  }

  getSortedItems(): Map<string, string[]> {
    return new Map([...this._items].sort(([key1], [key2]) => key1.localeCompare(key2)))
  }

  changed(item: string): this {
    return this.addItem('changed', item)
  }

  added(item: string): this {
    return this.addItem('added', item)
  }

  fixed(item: string): this {
    return this.addItem('fixed', item)
  }

  removed(item: string): this {
    return this.addItem('removed', item)
  }

  addItem(category: string, item: string): this {
    const normalizedCategory = category.trim().toLowerCase()
    const ary = this._items.get(normalizedCategory) || []
    this._items.set(normalizedCategory, ary)
    ary.push(item)

    return this
  }
}
