import { PaperClipIcon } from "@heroicons/react/20/solid";
import { useRouter } from "next/router";
import { useQuery } from "react-query";
import { api } from "../../../utils/api";

export default function UserInfoPage() {
  const router = useRouter();
  const id = typeof router.query?.id === "string" ? router.query.id : "";

  const { data, error, isLoading } = useQuery(
    ["users", id],
    async () => {
      console.log(router.isReady);
      const user = await api.get(`/user/${id}`);
      return user.data;
    },
    { enabled: id.length > 0 }
  );

  if (isLoading) {
    return <>Loading ...</>;
  }

  if (error) {
    console.log(error);
    return <>Error ...</>;
  }

  return (
    <div className="overflow-hidden bg-white shadow sm:rounded-lg">
      <div className="px-4 py-5 sm:px-6">
        <h3 className="text-base font-semibold leading-6 text-gray-900">
          {data?.name}
        </h3>
        <p className="mt-1 max-w-2xl text-sm text-gray-500">
          Dados pessoais
        </p>
      </div>
      <div className="border-t border-gray-200 px-4 py-5 sm:p-0">
        <dl className="sm:divide-y sm:divide-gray-200">
          <div className="py-4 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 sm:py-5">
            <dt className="text-sm font-medium text-gray-500">
            Email
            </dt>
            <dd className="mt-1 text-sm text-gray-900 sm:col-span-2 sm:mt-0">
            {data?.email}
            </dd>
          </div>
        </dl>
      </div>
    </div>
  );
}
