/*
 * Copyright (c) Johannes Grimm 2024.
 */
import EditUserForm from '@/app/ui/admin/user/edit-form';
import { auth } from '@/auth';
import BreadCrumb from '@/components/bread-crumb';
import { fetchRoles } from '@/lib/data';

import type { User, UsersInRoles } from '@prisma/client';
import type { Metadata } from 'next';

export const metadata: Metadata = {
  title: 'Create User',
};

export default async function Page() {
  const roles = await fetchRoles('');

  const session = await auth();
  if (!session || !session.user) throw new Error('Not authenticated');

  const user: { roles: UsersInRoles[] } & User = {
    createdAt: new Date(),
    displayName: '',
    email: '',
    id: '',
    lastLogin: null,
    password: '',
    roles: [],
  };

  return (
    <div className='flex min-h-screen w-full flex-col bg-muted/40'>
      <div className='flex flex-col sm:gap-4 sm:py-4 '>
        <header className='sticky top-0 flex h-14 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6'>
          <BreadCrumb />
        </header>
        <main className='grid flex-1 place-items-stretch items-start gap-4 p-4 sm:px-6 sm:py-0 md:gap-8'>
          <div className='size-full place-self-center lg:max-w-2xl'>
            <EditUserForm
              edit={false}
              id=''
              roles={roles}
              sessionUserId={session.user.id}
              user={user}
            />
          </div>
        </main>
      </div>
    </div>
  );
}
