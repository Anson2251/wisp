<script lang="ts" setup>
import { ref, watch, type PropType } from "vue";
import { NThing, NInput, NButton, NButtonGroup } from "naive-ui";
import Window from "./Window.vue";
import { useChatStore } from "../stores/chat";
import type { Conversation } from "../libs/types";

const props = defineProps({
  conversation: {
    type: Object as PropType<Conversation>,
    required: true,
  },
  onClose: {
    type: Function as PropType<() => void>,
    required: true,
  },
});

const chatStore = useChatStore();
const isEditingTitle = ref(false);
const isEditingDescription = ref(false);
const editedTitle = ref(props.conversation.name);
const editedDescription = ref(props.conversation.description || "");
const hasChanges = ref(false);

watch([editedTitle, editedDescription], ([newTitle, newDesc]) => {
  hasChanges.value =
    newTitle !== props.conversation.name ||
    newDesc !== (props.conversation.description || "");
});

const saveChanges = async () => {
  await chatStore.updateConversation(props.conversation.id, {
    name: editedTitle.value,
    description: editedDescription.value,
  });
  props.onClose();
};
</script>

<template>
  <window :title="'Conversation Info'" @close="onClose">
    <div class="info-content">
      <n-thing>
        <template #header>
          <n-input
            v-if="isEditingTitle"
            v-model:value="editedTitle"
            @blur="isEditingTitle = false"
            @keydown.enter="isEditingTitle = false"
            :autofocus="isEditingTitle"
          />
          <div v-else @click="isEditingTitle = true">
            {{ editedTitle }}
          </div>
        </template>

        <template #description>
          <n-input
            v-if="isEditingDescription"
            v-model:value="editedDescription"
            type="textarea"
            @blur="isEditingDescription = false"
            autofocus
          />
          <div v-else @click="isEditingDescription = true">
            {{ editedDescription || "No description" }}
          </div>
        </template>
      </n-thing>
    </div>

    <template #footer>
      <n-button-group>
        <n-button :disabled="!hasChanges" @click="saveChanges"> Save </n-button>
        <n-button @click="() => onClose()"> Cancel </n-button>
      </n-button-group>
    </template>
  </window>
</template>

<style scoped>
.info-content {
  padding: 16px;
}
</style>
