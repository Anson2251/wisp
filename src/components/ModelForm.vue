<script lang="ts" setup>
import {
  NCheckbox,
  NCheckboxGroup,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NSelect,
  NSpace,
} from "naive-ui";
import { ref, onMounted, watch } from "vue";
import {
  Model,
  ModelInfo,
  ModelMetadata,
  TextModelCapability,
  TextGenerationParams,
} from "../libs/types";

defineProps<{ model: Model | null }>();

const model = defineModel<Model | null>("model");

const modelTypes = [
  { label: "Text Generation", value: "text_generation" },
  { label: "Image Generation", value: "image_generation" },
  { label: "Embedding", value: "embedding" },
  { label: "Reranker", value: "reranker" },
  { label: "Audio", value: "audio" },
];

const metadata = ref<ModelMetadata>({
  name: "",
  display_name: "",
  description: "",
});

const modelType = ref<
  "text_generation" | "image_generation" | "embedding" | "reranker" | "audio"
>("text_generation");
const tokenizer = ref("");
const max_input_size = ref(2048);
const api_endpoint = ref("");
const textCapabilities = ref<TextModelCapability[]>([]);

onMounted(() => {
  if (model.value) {
    metadata.value = { ...model.value.metadata };
    modelType.value = model.value.model_info.type;
    tokenizer.value = model.value.tokenizer || "";
    max_input_size.value = model.value.max_input_size || 2048;
    api_endpoint.value = model.value.api_endpoint || "";

    switch (model.value.model_info.type) {
      case "text_generation":
        textParams.value = { ...model.value.model_info.configs.parameters };
        textCapabilities.value = [
          ...(model.value.model_info.configs.capabilities || []),
        ];
        break;
      case "image_generation":
        imageParams.value = { ...model.value.model_info.configs.parameters };
        break;
    }
  }
});

const modelInfo = ref<ModelInfo>({
  type: "text_generation",
  configs: {
    parameters: {
      temperature: 0.7,
      top_p: 1.0,
      max_tokens: 2048,
      presence_penalty: 0,
      frequency_penalty: 0,
      stop_sequences: [],
    },
    capabilities: [],
  },
});

const textParams = ref<TextGenerationParams>({
  temperature: 0.7,
  top_p: 1.0,
  max_tokens: 2048,
  presence_penalty: 0,
  frequency_penalty: 0,
  stop_sequences: [],
});

const imageParams = ref({
  width: 512,
  height: 512,
  steps: 50,
  cfg_scale: 7,
});

const updateModelInfo = () => {
  switch (modelType.value) {
    case "text_generation":
      modelInfo.value = {
        type: "text_generation",
        configs: {
          parameters: textParams.value,
          capabilities: textCapabilities.value,
        },
      };
      break;
    case "image_generation":
      modelInfo.value = {
        type: "image_generation",
        configs: {
          parameters: imageParams.value,
        },
      };
      break;
    case "embedding":
      modelInfo.value = {
        type: "embedding",
        configs: {
          parameters: {
            normalize: true,
            truncate: true,
          },
        },
      };
      break;
    case "reranker":
      modelInfo.value = {
        type: "reranker",
        configs: {
          parameters: {
            return_documents: true,
          },
        },
      };
      break;
    case "audio":
      modelInfo.value = {
        type: "audio",
      };
      break;
  }
};

watch(
  [
    modelType,
    metadata,
    textParams,
    imageParams,
    tokenizer,
    max_input_size,
    api_endpoint,
    textCapabilities,
  ],
  () => {
    updateModelInfo();
    model.value = {
      metadata: metadata.value,
      model_info: modelInfo.value,
      tokenizer: tokenizer.value,
      api_endpoint: api_endpoint.value,
      max_input_size: max_input_size.value,
    };
  },
  { deep: true }
);
</script>

<template>
  <keep-alive>
    <n-form>
      <!-- Metadata -->
      <n-form-item label="Name" required>
        <n-input v-model:value="metadata.name" />
      </n-form-item>
      <n-space item-style="flex-grow: 1;" :wrap="false">
        <n-form-item label="Display Name" required>
          <n-input v-model:value="metadata.display_name" />
        </n-form-item>
        <n-form-item label="Model Type" required>
          <n-select
            v-model:value="modelType"
            :options="modelTypes"
            @update:value="updateModelInfo"
          />
        </n-form-item>
      </n-space>
      <n-form-item label="Description">
        <n-input v-model:value="metadata.description" />
      </n-form-item>

      <!-- Text Generation Parameters -->
      <template v-if="modelType === 'text_generation'">
        <n-space item-style="flex-grow: 1;" :wrap="false">
          <n-form-item label="Temperature">
            <n-input-number
              v-model:value="textParams.temperature"
              :min="0"
              :max="2"
              :step="0.1"
            />
          </n-form-item>
          <n-form-item label="Top P">
            <n-input-number
              v-model:value="textParams.top_p"
              :min="0"
              :max="1"
              :step="0.1"
            />
          </n-form-item>
          <n-form-item label="Max Tokens">
            <n-input-number
              v-model:value="textParams.max_tokens"
              :min="1"
              :max="8192"
            />
          </n-form-item>
        </n-space>
      </template>

      <!-- Image Generation Parameters -->
      <template v-if="modelType === 'image_generation'">
        <n-space item-style="flex-grow: 1;" :wrap="false">
          <n-form-item label="Width">
            <n-input-number
              v-model:value="imageParams.width"
              :min="64"
              :max="2048"
            />
          </n-form-item>
          <n-form-item label="Height">
            <n-input-number
              v-model:value="imageParams.height"
              :min="64"
              :max="2048"
            />
          </n-form-item>
        </n-space>
        <n-space item-style="flex-grow: 1;" :wrap="false">
          <n-form-item label="Steps">
            <n-input-number
              v-model:value="imageParams.steps"
              :min="1"
              :max="150"
            />
          </n-form-item>
          <n-form-item label="CFG Scale">
            <n-input-number
              v-model:value="imageParams.cfg_scale"
              :min="1"
              :max="20"
            />
          </n-form-item>
        </n-space>
      </template>

      <!-- Text Generation Capabilities -->
      <template v-if="modelType === 'text_generation'">
        <n-form-item label="Capabilities">
          <n-checkbox-group v-model:value="textCapabilities">
            <n-space item-style="display: flex;">
              <n-checkbox value="FIM" label="FIM" />
              <n-checkbox value="ToolUse" label="Tool Use" />
              <n-checkbox value="Reasoning" label="Reasoning" />
            </n-space>
          </n-checkbox-group>
        </n-form-item>
      </template>

      <!-- Additional Model Fields -->
      <n-space item-style="flex-grow: 1;" :wrap="false" style="width: 100%;">
        <n-form-item label="Tokenizer">
          <n-input v-model:value="tokenizer" />
        </n-form-item>
        <n-form-item label="Max Input Size">
          <n-input-number v-model:value="max_input_size" :min="1" />
        </n-form-item>
      </n-space>
      <n-form-item label="API Endpoint">
        <n-input v-model:value="api_endpoint" />
      </n-form-item>

      <!-- Actions -->
      <n-space justify="end"> </n-space>
    </n-form>
  </keep-alive>
</template>
