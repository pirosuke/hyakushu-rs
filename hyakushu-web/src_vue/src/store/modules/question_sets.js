import QuestionSetAPI from "../../api/question_sets"

export default {
    namespaced: true,
    state: {
        questionSetInTrial: {},
    },
    mutations: {
        setQuestionSet(state, payload) {
            state.questionSetInTrial = payload.questionSet;
        },
    },
    actions: {
        async generateQuestionSet(context, payload) {
            const questionSet = await QuestionSetAPI.generateQuestionSet(payload.player_id)
            context.commit("setQuestionSet", {
                questionSet,
            });
        },

        async addAnswerLog(context, payload) {
            await QuestionSetAPI.addAnswerLog(
                payload.player_id,
                payload.question_set_id,
                payload.waka_id,
                payload.answered_correctly,
            )
        }
    },
}