<template lang="pug">
  v-card(
    style="width: 90%; height: 550px"
  )
    v-card-text
        div(class="d-flex flex-row-reverse ma-2 pa-2")
            v-btn(
                :icon="edit ? 'mdi-content-save' : 'mdi-pencil'"
                @click="edit = !edit"
            )
        v-row
            v-col(cols="2")
                template(v-if="selectedKiller.Data")
                    v-img(
                      style="width: 400px; height: 300px;"
                      :src="`data:image/png;base64,${selectedKiller.Data}`"
                      contain
                    )
                v-combobox(
                      v-if="edit"
                      v-model="selectedKiller"
                      label="Choose Killer"
                      :items="killerPortraits"
                      item-title="Name"
                      auto-select-first="exact"
                    )
            v-col(cols="8")
                v-row(v-for="n in 5" :key="n" class="mb-2")
                    template(v-if="n<5")
                        v-col(cols="2")
                            h4 {{Team1}}
                        v-combobox(
                          v-if="edit"
                          v-model="selectedPerks[n-1]"
                          label="Surv perks"
                          :items="perks.Surv"
                          chips
                          item-title="Name"
                          auto-select-first="exact"
                          :rules="[maxPerkRule]"
                          multiple
                        )
                    template(v-else)
                        v-col(cols="2")
                            h4 {{Team2}}
                        v-combobox(
                          v-if="edit"
                          v-model="selectedPerks[4]"
                          label="Killer perks"
                          :items="perks.Killer"
                          chips
                          item-title="Name"
                          auto-select-first="exact"
                          :rules="[maxPerkRule]"
                          multiple
                        )
                    
                    
</template>
<script setup>
import { invoke } from "@tauri-apps/api/core";
import { ref, onMounted } from "vue";

const selectedKiller = ref("")
const killerPortraits = ref([])
const perks = ref({});
const edit = ref(true)
const selectedPerks = ref([[],[],[],[],[]])

defineProps({
    Team1: String,
    Team2: String
})

const maxPerkRule = (value) => {
    if(value.length > 4) {
        return "You cant select more than 4 perks, silly goose x3"
    }
    return true
}

async function getPerks() {
  let killerPerks = await invoke("getIconData", {path: "perks/Killer"})
  let survPerks = await invoke("getIconData", {path: "perks/Surv"})
  perks.value = {
    "Killer": killerPerks,
    "Surv": survPerks
  }
  killerPortraits.value = await invoke("getIconData", {path: "KillerIcons"})
}
onMounted(async () => {
  await getPerks();
});
</script>