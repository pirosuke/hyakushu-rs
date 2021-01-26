<template>
  <v-container>
    <v-card>
      <v-card-title>
        {{questionNumber}}/{{totalQuestionNum}} {{targetQuestion.waka.kamino_ku}}
        <v-spacer></v-spacer>
      </v-card-title>

      <v-card-title v-if="showWrongAnswerMessage">
        残念! 正解は 「{{targetQuestion.waka.shimono_ku}}」 です。
      </v-card-title>

      <v-card-title v-if="showCorrectAnswerMessage">
        正解!
      </v-card-title>

      <v-card-text>
        <v-container>
          <v-form
            ref="form"
            >
            <v-row>
              <v-col cols="12" sm="12" md="12">
                <v-radio-group v-model="selectedAnswer">
                  <v-radio :value="targetQuestion.answer_choices[0]">
                    <template v-slot:label>
                      <div>{{targetQuestion.answer_choices[0]}}</div>
                    </template>
                  </v-radio>
                  <v-radio :value="targetQuestion.answer_choices[1]">
                    <template v-slot:label>
                      <div>{{targetQuestion.answer_choices[1]}}</div>
                    </template>
                  </v-radio>
                  <v-radio :value="targetQuestion.answer_choices[2]">
                    <template v-slot:label>
                      <div>{{targetQuestion.answer_choices[2]}}</div>
                    </template>
                  </v-radio>
                  <v-radio :value="targetQuestion.answer_choices[3]">
                    <template v-slot:label>
                      <div>{{targetQuestion.answer_choices[3]}}</div>
                    </template>
                  </v-radio>
                </v-radio-group>
              </v-col>
            </v-row>
          </v-form>
        </v-container>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="blue darken-1" v-if="mode === 'question'" @click="handleAnswer">答える</v-btn>
        <v-btn color="blue darken-1" v-if="mode === 'next'" @click="nextQuestion">次へ</v-btn>
        <v-btn color="blue darken-1" v-if="editedIndex === -1" @click="addItem">結果を見る</v-btn>
      </v-card-actions>
    </v-card>
  </v-container>
</template>

<script>
  import { mapGetters, mapState } from 'vuex'

  export default {
    data () {
      return {
        questionSet: {},
        totalQuestionNum: 10,
        questionNumber: 0,
        targetQuestion: {
          waka: {
            waka_id: 0,
            kamino_ku: "",
            shimono_ku: "",
            yomi_bito: "",
          },
          answer_choices: ["","","",""],
        },
        selectedAnswer: "",
        showWrongAnswerMessage: false,
        showCorrectAnswerMessage: false,
        mode: "question",
      }
    },

    watch: {
    },

    mounted() {
      this.$store.dispatch('question_sets/generateQuestionSet', {
        player_id: '99'
      }).then(() => {
        this.questionSet = this.$store.state.question_sets.questionSetInTrial;
        this.nextQuestion()
      })
    },
    computed: {
        ...mapState({
        }),
    },
    methods: {
      handleAnswer() {
        this.mode = "next"
        let isCorrectAnswer = false
        console.log(this.targetQuestion)
        console.log(this.selectedAnswer)
        if (this.targetQuestion.waka.shimono_ku === this.selectedAnswer) {
          isCorrectAnswer = true
        }

        if (isCorrectAnswer) {
          this.showCorrectAnswerMessage = true
        } else {
          this.showWrongAnswerMessage = true
        }

        this.$store.dispatch('question_sets/addAnswerLog', {
          player_id: '99',
          question_set_id: this.questionSet.question_set_id,
          waka_id: this.targetQuestion.waka.waka_id,
          answered_correctly: isCorrectAnswer,
        })
      },
      nextQuestion () {
        this.mode = "question"
        this.showCorrectAnswerMessage = false
        this.showWrongAnswerMessage = false
        this.selectedAnswer = ""
        this.questionNumber += 1
        this.targetQuestion = this.questionSet.questions[this.questionNumber]
      },
    }
  }
</script>
