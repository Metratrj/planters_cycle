'use client';

import { UserModel } from '@/prisma/zod';
import { zodResolver } from '@hookform/resolvers/zod';
import { User } from '@prisma/client';
import { redirect } from 'next/dist/server/api-utils';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { useToast } from '@/components/ui/use-toast';

import { editUser, redirectToUsers } from '@/lib/actions';

const editUserSchema = UserModel.partial({
  id: true,
  createdAt: true,
  lastLogin: true,
  password: true,
});

export default function EditUserForm({ id, user }: { id: string; user: User }) {
  const form = useForm<z.infer<typeof editUserSchema>>({
    resolver: zodResolver(editUserSchema),
    defaultValues: {
      displayName: user.displayName,
      email: user.email,
    },
  });

  const { toast } = useToast();

  async function onSubmit(values: z.infer<typeof editUserSchema>) {
    try {
      await editUser(id, values);
      toast({
        title: 'Success 🎉',
        description: `Updated user ${values.displayName}.`,
      });
      redirectToUsers();
    } catch (error) {
      toast({
        title: 'Uh oh! Something went wrong.',
        description: `There was a problem with your request.\n${error}`,
      });
    }
  }

  return (
    <>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
          <Card>
            <CardHeader>
              <CardTitle>Edit User</CardTitle>
              <CardDescription>
                Edit the user&apos;s information below.
              </CardDescription>
            </CardHeader>
            <CardContent className='grid gap-4'>
              <div className='grid gap-2'>
                <FormField
                  control={form.control}
                  name='displayName'
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Name</FormLabel>
                      <FormControl>
                        <Input placeholder='Max Mustermann' {...field} />
                      </FormControl>
                      <FormDescription>
                        This is your public display name.
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <div className='grid gap-2'>
                <FormField
                  control={form.control}
                  name='email'
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>eMail</FormLabel>
                      <FormControl>
                        <Input placeholder='dev@r4p1d.xyz' {...field} />
                      </FormControl>
                      <FormDescription>
                        This is your email address.
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
              <div className='grid gap-2'>
                <FormField
                  control={form.control}
                  name='password'
                  render={({ field }) => (
                    <FormItem>
                      <FormLabel>Password</FormLabel>
                      <FormControl>
                        <Input
                          type='password'
                          placeholder='********'
                          {...field}
                        />
                      </FormControl>
                      <FormDescription>
                        This is your private password.
                      </FormDescription>
                      <FormMessage />
                    </FormItem>
                  )}
                />
              </div>
            </CardContent>
            <CardFooter>
              <Button type='submit' className='w-full'>
                Save
              </Button>
            </CardFooter>
          </Card>
        </form>
      </Form>
    </>
  );
}
