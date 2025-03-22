<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import BracketComponent from "./components/BracketComponent.vue"

const perks = ref({});
const killerPortraits = ref([])
const T1 = ref("");
const T2 = ref("");
const created = ref(false);
const selectedKiller = ref("")

async function debugLog() {
  console.log(selectedKiller)
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

<template lang="pug">
  v-app
    v-main
      template(v-if="!created")
        div(class="d-flex justify-center" style="margin-top: 15%; margin-bottom: 2%")
          h1 Create new match 

        v-row(class="d-flex justify-center")
          v-col(
            cols="5"
            sm="5"
          )
            v-text-field(
              label="Team 1"
              v-model="T1",
              
              variant="outlined"
            )
          v-col(
            cols="5"
            sm="5"
          )
            v-text-field(
              label="Team 2"
              v-model="T2",
              variant="outlined"
            )
        div(class="d-flex justify-center")
          v-btn(
            class="text-none"
            variant="text"
            @Click="created = true"
            border
          ) Create match
          
      template(v-if="created")
        v-row(class="d-flex justify-center align-center")
          v-col(
              cols="3"
              sm="3"
              class="text-center"
          )
            h2 {{T1}}
          v-col(
              cols="2"
              sm="2"
              class="text-center"
          )
            h1 VS.
          v-col(
              cols="3"
              sm="3"
              class="text-center"
          )
            h2 {{T2}}
        div(class="d-flex justify-center")
          BracketComponent(
            :Team1="T1",
            :Team2="T2"
          )
        div(class="d-flex justify-center" style="margin-top: 2%")
          BracketComponent(
            :Team1="T1",
            :Team2="T2"
          )
        div(class="d-flex justify-center" style="margin-top: 2%")
          BracketComponent(
            :Team1="T1",
            :Team2="T2"
          )
                    
                
</template>

<style scoped>

</style>
