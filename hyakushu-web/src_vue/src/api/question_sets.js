import axios from "axios"

const apiServer = axios.create({
    baseURL: process.env.VUE_APP_AEC_BASE_URL,
})

export default {
    async generateQuestionSet(playerID) {
        const res = await apiServer.post(
            "/question_sets/generate",
            {
                player_id: playerID,
            },
        )
        return res.data
    },

    async addAnswerLog(playerID, questionSetID, wakaID, answeredCorrectly) {
        const res = await apiServer.post(
            "/question_sets/add_answer",
            {
                player_id: playerID,
                question_set_id: questionSetID,
                waka_id: wakaID,
                answered_correctly: answeredCorrectly,
            },
        )
    }
}