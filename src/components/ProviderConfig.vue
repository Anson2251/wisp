<script lang="ts" setup>
import {
  NButton,
  NCard,
  NDataTable,
  NDrawer,
  NDrawerContent,
  NForm,
  NFormItem,
  NInput,
  NSpace,
  NIcon,
  useMessage,
  useDialog,
} from "naive-ui";
import { Add20Regular, Delete16Regular, Edit16Regular } from "@vicons/fluent";
import { ref, h, onMounted, watch } from "vue";
import { Provider, Model } from "../libs/types";
import { debounce } from "lodash";
import { getCredential, setCredential } from "../libs/commands";
import ModelForm from "./ModelForm.vue";
import { useProviderStore } from "../stores/provider";

const props = defineProps<{
  provider: Provider;
}>();

const dialog = useDialog();
const providers = useProviderStore();
const message = useMessage();
const showAddModel = ref(false);
const showEditModel = ref(false);
const selectedModel = ref<Model | null>(null);
const providerForm = ref<Omit<Provider, "models">>({ ...props.provider });
const apiKey = ref("");
let storedApiKey: string | null = null;

const modelColumns = [
  {
    title: "Name",
    key: "metadata.name",
  },
  {
    title: "Displayed Name",
    key: "metadata.display_name",
  },
  {
    title: "Type",
    key: "model_info.type",
  },
  {
    title: "Actions",
    key: "actions",
    render(row: Model) {
      return h(NSpace, {}, () => [
        h(NButton, {
          type: "primary",
          size: "small",
          quaternary: true,
          circle: true,
          onClick: () => {
            selectedModel.value = row;
            showEditModel.value = true;
          },
          renderIcon: () => h(NIcon, null, { default: () => h(Edit16Regular) }),
        }),
        h(NButton, {
          type: "error",
          size: "small",
          quaternary: true,
          circle: true,
          onClick: () => handleDeleteModel(row.metadata.name),
          renderIcon: () =>
            h(NIcon, { size: 16 }, { default: () => h(Delete16Regular) }),
        }),
      ]);
    },
  },
];

const handleUpdateProvider = async () => {
  try {
    await providers.updateProvider(props.provider.name, {
      ...providerForm.value,
      models: props.provider.models,
    });
    if (storedApiKey != apiKey.value && apiKey.value) {
      storedApiKey = apiKey.value;
      await setCredential(props.provider.name, apiKey.value);
    }
    message.success("Provider updated");
  } catch (e) {
    message.error(`Failed to update provider: ${e}`);
  }
};

const handleAddModel = async (model: Model) => {
  try {
    await providers.addModel(props.provider.name, model);
    message.success("Model added");
    showAddModel.value = false;
  } catch (e) {
    message.error(`Failed to add model: ${e}`);
  }
};

const handleUpdateModel = async (model: Model) => {
  try {
    await providers.updateModel(
      props.provider.name,
      model.metadata.name,
      model
    );
    message.success("Model updated");
    showEditModel.value = false;
    selectedModel.value = null;
  } catch (e) {
    message.error(`Failed to update model: ${e}`);
  }
};

const handleDeleteModel = async (modelName: string) => {
  try {
    const confirmed = await new Promise((resolve) => {
      dialog.warning({
        title: "Delete Model",
        content: `Are you sure you want to delete the model "${props.provider.models.find(m => m.metadata.name === modelName)?.metadata.display_name || modelName}" in "${props.provider.display_name}" provider"? This process cannot be undone.`,
        positiveText: 'Confirm',
        negativeText: 'Cancel',
        onPositiveClick: () => resolve(true),
        onNegativeClick: () => resolve(false),
      });
    });

    if (!confirmed) return;

    await providers.deleteModel(props.provider.name, modelName);
    message.success("Model deleted");
  } catch (e) {
    message.error(`Failed to delete model: ${e}`);
  }
};

onMounted(async () => {
  try {
    apiKey.value = await getCredential(props.provider.name);
    if (apiKey.value) {
      storedApiKey = apiKey.value;
    }
  } catch (e) {
    console.error("Failed to load API key:", e);
  }
});

watch(
  providerForm,
  debounce(() => {
    handleUpdateProvider();
  }, 500),
  { deep: true }
);
</script>

<template>
  <div class="container">
    <n-space vertical>
      <!-- Provider Details -->
      <n-card title="Provider Details" size="small" :bordered="false">
        <n-form>
          <n-space
            horizontal
            align="center"
            item-style="flex-grow: 1;"
            :wrap="false"
            ><n-form-item label="Name">
              <n-input v-model:value="providerForm.name" />
            </n-form-item>
            <n-form-item label="Display Name">
              <n-input v-model:value="providerForm.display_name" />
            </n-form-item>
          </n-space>
          <n-form-item label="Base URL">
            <n-input v-model:value="providerForm.base_url" />
          </n-form-item>
          <n-form-item label="API Key">
            <n-input
              v-model:value="apiKey"
              type="password"
              placeholder="Enter API key"
              :show-password-on="'click'"
              @blur="
                () => {
                  if (storedApiKey != apiKey) handleUpdateProvider();
                }
              "
            />
          </n-form-item>
        </n-form>
      </n-card>

      <!-- Model Management -->
      <n-card title="Models" size="small" :bordered="false">
        <n-space vertical>
          <n-data-table
            :columns="modelColumns"
            :data="provider.models"
            :bordered="false"
          />
        </n-space>
        <template #header-extra>
          <n-button @click="() => {
            showAddModel = true
            selectedModel = null
          }" tertiary circle>
            <template #icon>
              <n-icon :size="20">
                <Add20Regular />
              </n-icon>
            </template>
          </n-button>
        </template>
      </n-card>

      <!-- Model Drawer -->
      <n-drawer
        :show="showAddModel || showEditModel"
        @update:show="
          (val) => {
            if (!val) {
              showAddModel = false;
              showEditModel = false;
            }
          }
        "
        :width="600"
      >
        <n-drawer-content :title="showEditModel ? 'Edit Model' : 'Add Model'">
          <ModelForm v-model:model="selectedModel" />
          <template #footer>
            <n-space horizontal>
              <n-button
                @click="
                  () => {
                    showAddModel = false;
                    showEditModel = false;
                    selectedModel = null;
                  }
                "
                >Cancel</n-button
              >
              <n-button
                type="primary"
                @click="
                  () => {
                    if (!selectedModel) return;
                    if (showAddModel) handleAddModel(selectedModel);
                    else handleUpdateModel(selectedModel);
                  }
                "
              >
                {{ showEditModel ? "Update" : "Add" }} Model
              </n-button>
            </n-space>
          </template>
        </n-drawer-content>
      </n-drawer>
    </n-space>
  </div>
</template>

<style scoped>
.container {
  padding: 8px;
  box-sizing: border-box;
  width: 100%;
  height: 100%;
}
</style>
