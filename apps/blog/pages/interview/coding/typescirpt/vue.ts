type GetComputed<C> = C extends Record<string, (...args: any[]) => any>
  ? { [S in keyof C]: ReturnType<C[S]> }
  : never

declare function SimpleVue<D, C, M>(
  options: {
    data: () => D,
    computed: C,
    methods: M,
  } & ThisType<D & M & GetComputed<C>>
): any
// declare function SimpleVue(options: any): any

const instance = SimpleVue({
  data() {
    return {
      firstname: 'Type',
      lastname: 'Challenges',
      amount: 10,
    }
  },
  computed: {
    fullname() {
      return this.firstname + ' ' + this.lastname
    }
  },
  methods: {
    hi() {
      alert(this.fullname.toLowerCase())
    }
  }
})

const tuple = ['tesla', 'model 3', 'model X', 'model Y'] as const
type a = typeof tuple
