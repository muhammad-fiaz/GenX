// src/lib/sidebarItems.ts

export interface SidebarItem {
	name: string;
	href: string;
	svg: string; // Inline SVG string
}

export const sidebarItems: SidebarItem[] = [
	{
		name: 'Chat',
		href: '/chat',
		svg: `<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
				<path stroke-linecap="round" stroke-linejoin="round" d="m10 7l-.516 1.394c-.676 1.828-1.014 2.742-1.681 3.409s-1.581 1.005-3.409 1.681L3 14l1.394.516c1.828.676 2.742 1.015 3.409 1.681s1.005 1.581 1.681 3.409L10 21l.516-1.394c.676-1.828 1.015-2.742 1.681-3.409s1.581-1.005 3.409-1.681L17 14l-1.394-.516c-1.828-.676-2.742-1.014-3.409-1.681s-1.005-1.581-1.681-3.409z" />
			</svg>`
	},
	{
		name: 'Config',
		href: '/config',
		svg: `\t<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 transition-colors" viewBox="0 0 24 24"><g fill="none" stroke="currentColor" stroke-width="2"><path d="M3.082 13.945c-.529-.95-.793-1.426-.793-1.945c0-.519.264-.994.793-1.944L4.43 7.63l1.426-2.381c.559-.933.838-1.4 1.287-1.66c.45-.259.993-.267 2.08-.285L12 3.26l2.775.044c1.088.018 1.631.026 2.08.286c.45.26.73.726 1.288 1.659L19.57 7.63l1.35 2.426c.528.95.792 1.425.792 1.944c0 .519-.264.994-.793 1.944L19.57 16.37l-1.426 2.381c-.559.933-.838 1.4-1.287 1.66c-.45.259-.993.267-2.08.285L12 20.74l-2.775-.044c-1.088-.018-1.631-.026-2.08-.286c-.45-.26-.73-.726-1.288-1.659L4.43 16.37z"/><circle cx="12" cy="12" r="3"/></g></svg>
`
	}
];
