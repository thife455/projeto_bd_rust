
import { useRouter } from "next/router";
import { useState } from "react";
import { useMutation } from "react-query";
import { api } from "../../../utils/api";

export default function DepositUserPage() {
  const router = useRouter();

  const [amount, setAmount] = useState<number>();

  const { mutate } = useMutation({
    mutationFn: async () => { return await api.post(`/user/${router.query.id}/deposit`, { amount: amount }) },
    onMutate() {
      router.back()
    },
  })
  return (
    <>
      <div className="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
        <div className="sm:mx-auto sm:w-full sm:max-w-md">
          <h2 className="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">Depositar para usuario</h2>
          <p className="mt-2 text-center text-sm text-gray-600">
          </p>
        </div>

        <div className="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
          <div className="bg-white px-4 py-8 shadow sm:rounded-lg sm:px-10">
            <form className="space-y-6" onSubmit={(e) => { e.preventDefault(); mutate() }}>
              <div>
                <label htmlFor="email" className="block text-sm font-medium leading-6 text-gray-900">
                  Quantidade
                </label>
                <div className="mt-2">
                  <input
                    id="number"
                    name="number"
                    type="number"
                    onChange={(e) => { setAmount(parseInt(e.target.value)) }}
                    required
                    className="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                  />
                </div>
              </div>

              <div>
                <button
                  type="submit"
                  className="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                >
                  Criar
                </button>
              </div>
            </form>
          </div>
        </div>
      </div>
    </>
  )
}
