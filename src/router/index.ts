import { createRouter, createWebHistory } from 'vue-router'
import ChatView from '../views/ChatView.vue'
import PalsView from '../views/PalsView.vue'
import ProvidersView from '../views/ProvidersView.vue'

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: "/",
			redirect: { path: "/chat" },
		},
		{
			path: '/chat',
			name: 'chat',
			component: ChatView
		},
		{
			path: '/pals',
			name: 'pals',
			component: PalsView
		},
		{
			path: '/providers',
			name: 'providers',
			component: ProvidersView
		}
	]
})

export default router
