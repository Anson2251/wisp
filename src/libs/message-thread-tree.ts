export class MessageThreadTree {
	private tree: Map<string, readonly string[]> = new Map();

	public addNode(id: string, parentId?: string | null, children: readonly string[] = []): void {
		if (parentId) {
			const siblings = this.tree.get(parentId) ?? [];
			if (!siblings.includes(id)) this.tree.set(parentId, [...siblings, id]);
		}
		this.tree.set(id, children)
	}

	public clear(): void {
		this.tree.clear();
	}

	public removeNode(id: string): void {
		const parentId = this.getParentId(id);
		const children = this.tree.get(id) ?? [];

		if (!parentId && children.length > 1) {
			throw Error(`[MessageThreadTree] Cannot remove root node ${id} with multiple children. Please remove children first.`)
		}

		for (const child of children) {
			this.updateParentId(child, parentId);
		}

		this.tree.delete(id);
		for (const [parentId, nodes] of this.tree) {
			this.tree.set(parentId, nodes.filter(node => node !== id));
		}
	}

	public updateParentId(id: string, newParentId?: string): void {
		const oldParentId = this.getParentId(id);
		if (oldParentId) {
			this.removeNode(id);
			this.addNode(id, newParentId);
		}
	}

	public updateChildren(id: string, newChildren: string[]): void {
		this.tree.set(id, newChildren);
	}

	public hasNode(id: string): boolean {
		return this.tree.has(id);
	}

	public getNodeDepth(id: string): number {
		if (!this.hasNode(id)) return -1;

		let depth = 0;
		let currentId = id;
		while (this.getParentId(currentId)) {
			depth++;
			currentId = this.getParentId(currentId) ?? "";
		}
		return depth;
	}

	public getNodeSiblingOrder(id: string): number {
		const parentId = this.getParentId(id);
		if (!parentId) return -1;
		const siblings = this.getChildren(parentId);
		return siblings.indexOf(id);
	}

	public getNodeSiblingNumber(id: string): number {
		const siblings = this.getChildren(this.getParentId(id) ?? "");
		return siblings.length;
	}

	public getChildren(parentId: string): readonly string[] {
		return this.tree.get(parentId) ?? [];
	}

	public getParentId(id: string): string | undefined {
		for (const [parentId, siblings] of this.tree) {
			if (siblings.includes(id)) {
				return parentId;
			}
		}
		return undefined;
	}
}
export default MessageThreadTree;
