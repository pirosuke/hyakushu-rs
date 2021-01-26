<template>
  <v-app>

    <v-navigation-drawer
      app
    >
      <v-list dense>
        <v-list-item link @click="$router.push({path: `/Trial`})">
          <v-list-item-action>
            <v-icon>mdi-wan</v-icon>
          </v-list-item-action>
          <v-list-item-content>
            <v-list-item-title>特訓開始</v-list-item-title>
          </v-list-item-content>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar
      app
      color="indigo"
      dark
    >
      <v-toolbar-title class="headline">
        <span>百人一首特訓ツール</span>
      </v-toolbar-title>
      <v-spacer></v-spacer>
    </v-app-bar>

    <v-main>
      <v-container fluid class="fill-height">
        <v-layout row>
          <v-flex grow pa-1>
            <router-view/>
          </v-flex>
        </v-layout>
      </v-container>
    </v-main>
    <v-footer
      color="indigo"
      app
    >
      <span class="white--text">&copy; 2021</span>
    </v-footer>
  </v-app>
</template>

<script>
import { mapGetters, mapState } from 'vuex'

export default {
  name: 'App',
  components: {
  },
  data () {
    return {
    }
  },
  computed: {
    ...mapState({
    })
  },
}
</script>
