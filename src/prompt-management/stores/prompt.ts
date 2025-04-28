import { defineStore } from 'pinia'

export const usePromptStore = defineStore('prompt', {
  state: () => ({
    selectedSystemPromptId: 'default' as 'default' | 'technical' | 'creative',
    systemPrompts: {
      'default': 'You are a helpful AI assistant.',
      'technical': 'You are a technical expert. Provide detailed, accurate answers.',
      'creative': 'You are a creative writer. Generate imaginative and engaging content.'
    } as Record<'default' | 'technical' | 'creative', string>
  }),
  getters: {
    selectedSystemPrompt: (state) => state.systemPrompts[state.selectedSystemPromptId]
  },
  actions: {
    selectSystemPrompt(id: 'default' | 'technical' | 'creative') {
      this.selectedSystemPromptId = id
    }
  }
})
