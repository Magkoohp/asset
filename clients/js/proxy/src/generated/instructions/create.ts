/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Context,
  Pda,
  PublicKey,
  Signer,
  TransactionBuilder,
  transactionBuilder,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  mapSerializer,
  string,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { resolveProxiedAsset } from '../../hooked';
import {
  PickPartial,
  ResolvedAccount,
  ResolvedAccountsWithIndices,
  getAccountMetasAndSigners,
} from '../shared';

// Accounts.
export type CreateInstructionAccounts = {
  /** The proxied asset (seeds: `[stub]`) */
  asset?: PublicKey | Pda;
  /** The ephemeral stub to derive the address of the asset */
  stub: Signer;
  /** The owner of the asset */
  owner: PublicKey | Pda;
  /** The account paying for the storage fees */
  payer?: Signer;
  /** System program */
  systemProgram?: PublicKey | Pda;
  /** Nifty Asset program */
  niftyAssetProgram?: PublicKey | Pda;
};

// Data.
export type CreateInstructionData = { discriminator: number; name: string };

export type CreateInstructionDataArgs = { name: string };

export function getCreateInstructionDataSerializer(): Serializer<
  CreateInstructionDataArgs,
  CreateInstructionData
> {
  return mapSerializer<CreateInstructionDataArgs, any, CreateInstructionData>(
    struct<CreateInstructionData>(
      [
        ['discriminator', u8()],
        ['name', string()],
      ],
      { description: 'CreateInstructionData' }
    ),
    (value) => ({ ...value, discriminator: 0 })
  ) as Serializer<CreateInstructionDataArgs, CreateInstructionData>;
}

// Extra Args.
export type CreateInstructionExtraArgs = { version?: number };

// Args.
export type CreateInstructionArgs = PickPartial<
  CreateInstructionDataArgs & CreateInstructionExtraArgs,
  'version'
>;

// Instruction.
export function create(
  context: Pick<Context, 'eddsa' | 'identity' | 'payer' | 'programs'>,
  input: CreateInstructionAccounts & CreateInstructionArgs
): TransactionBuilder {
  // Program ID.
  const programId = context.programs.getPublicKey(
    'proxy',
    'Proxy11111111111111111111111111111111111111'
  );

  // Accounts.
  const resolvedAccounts = {
    asset: {
      index: 0,
      isWritable: true as boolean,
      value: input.asset ?? null,
    },
    stub: { index: 1, isWritable: false as boolean, value: input.stub ?? null },
    owner: {
      index: 2,
      isWritable: false as boolean,
      value: input.owner ?? null,
    },
    payer: {
      index: 3,
      isWritable: true as boolean,
      value: input.payer ?? null,
    },
    systemProgram: {
      index: 4,
      isWritable: false as boolean,
      value: input.systemProgram ?? null,
    },
    niftyAssetProgram: {
      index: 5,
      isWritable: false as boolean,
      value: input.niftyAssetProgram ?? null,
    },
  } satisfies ResolvedAccountsWithIndices;

  // Arguments.
  const resolvedArgs: CreateInstructionArgs = { ...input };

  // Default values.
  if (!resolvedAccounts.asset.value) {
    resolvedAccounts.asset = {
      ...resolvedAccounts.asset,
      ...resolveProxiedAsset(
        context,
        resolvedAccounts,
        resolvedArgs,
        programId,
        true
      ),
    };
  }
  if (!resolvedAccounts.systemProgram.value) {
    if (resolvedAccounts.payer.value) {
      resolvedAccounts.systemProgram.value = context.programs.getPublicKey(
        'systemProgram',
        '11111111111111111111111111111111'
      );
      resolvedAccounts.systemProgram.isWritable = false;
    }
  }
  if (!resolvedAccounts.niftyAssetProgram.value) {
    resolvedAccounts.niftyAssetProgram.value = context.programs.getPublicKey(
      'niftyAsset',
      'AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'
    );
    resolvedAccounts.niftyAssetProgram.isWritable = false;
  }
  if (!resolvedArgs.version) {
    resolvedArgs.version = 1;
  }

  // Accounts in order.
  const orderedAccounts: ResolvedAccount[] = Object.values(
    resolvedAccounts
  ).sort((a, b) => a.index - b.index);

  // Keys and Signers.
  const [keys, signers] = getAccountMetasAndSigners(
    orderedAccounts,
    'programId',
    programId
  );

  // Data.
  const data = getCreateInstructionDataSerializer().serialize(
    resolvedArgs as CreateInstructionDataArgs
  );

  // Bytes Created On Chain.
  const bytesCreatedOnChain = 0;

  return transactionBuilder([
    { instruction: { keys, programId, data }, signers, bytesCreatedOnChain },
  ]);
}