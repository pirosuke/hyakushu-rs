import Vue from "vue";
import Vuex from "vuex";
import question_sets from './modules/question_sets'

Vue.use(Vuex);

export default new Vuex.Store({
  modules: {
    question_sets,
  }
});
