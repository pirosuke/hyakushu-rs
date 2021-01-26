<template>
  <v-container>
    <v-card>
      <v-card-title>
        百人一首特訓場へようこそ
        <v-spacer></v-spacer>
      </v-card-title>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="blue darken-1" @click="$router.push({path: `/Trial`})">特訓を開始する</v-btn>
          <v-btn color="blue darken-1" @click="addItem">成績を見る</v-btn>
        </v-card-actions>
    </v-card>
  </v-container>
</template>

<script>
  import { mapGetters, mapState } from 'vuex'

  export default {
    data () {
      return {
      }
    },

    watch: {
    },

    mounted() {
    },
    computed: {
        ...mapState({
        })
    },
    methods: {
      addItem () {
      },
    }
  }
</script>
