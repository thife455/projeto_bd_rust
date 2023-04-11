import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware'


interface UserId {
  id: string,
  setId: (id: string) => void
}

const useUserIdStore = create<UserId>()(
  devtools(
    persist(
      (set) => ({
        id: "",
        setId: (id) => set(() => ({ id: id })),
      }),
      {
        name: 'user-id',
      }
    )
  )
)

export { useUserIdStore }
