import { useRouter } from "next/router";


type User = {
    id: number;
    name: string;
    email: string;
}

type UserTableProps = {
    data?: User[];
}

const UserTable = ({ data }: UserTableProps) => {
    const router = useRouter();

    if (data.length === 0) {
        return (<>
            <div className="flex flex-col items-center">
                <p className="m-4">Essa academia nao tem usuarios :-(</p>
                <button
                    type="button"
                    className="rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                    onClick={() => router.back()}
                >
                    Voltar
                </button>
            </div>
        </>)
    }

    return (
        <>
            <table className="min-w-full divide-y divide-gray-200">
                <thead>
                    <tr>
                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                            Nome
                        </th>
                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                            Email
                        </th>
                    </tr>
                </thead>
                <tbody>
                    {data?.map((user) => (
                        <tr key={user.id}>
                            <td className="px-6 py-4 whitespace-nowrap">{user.name}</td>
                            <td className="px-6 py-4 whitespace-nowrap">{user.email}</td>
                            <td className="px-6 py-4 whitespace-nowrap">
                                <button
                                    onClick={() => {
                                        router.push(`users/${user.id}`);
                                    }}
                                    className="bg-blue-500 text-white rounded-md"
                                >
                                    <p className="m-3">Detalhes</p>
                                </button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </>
    );
};

export default UserTable;
