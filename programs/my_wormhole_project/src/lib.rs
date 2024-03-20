use anchor_lang::prelude::*;

pub use context::*;
pub use state::*;

pub mod context;
pub mod state;

declare_id!("F1tEp6JUoq255o2nWr1nEZJcQtv5dKWryLe6scHLqmPd");

#[program]
pub mod my_wormhole_project {
    use super::*;
    use anchor_lang::solana_program;
    use wormhole_anchor_sdk::wormhole;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let config = &mut ctx.accounts.config;

        config.owner = ctx.accounts.owner.key();

        // Set Wormhole related addresses.
        {
            let wormhole = &mut config.wormhole;

            wormhole.bridge = ctx.accounts.wormhole_bridge.key();

            wormhole.fee_collector = ctx.accounts.wormhole_fee_collector.key();

            wormhole.sequence = ctx.accounts.wormhole_sequence.key();
        }

        // Set default values for posting Wormhole messages.
        config.batch_id = 0;

        config.finality = wormhole::Finality::Confirmed as u8;

        ctx.accounts.wormhole_emitter.bump = *ctx
            .bumps
            .get("wormhole_emitter")
            .ok_or(WormholeError::BumpNotFound)?;
        {

            let fee = ctx.accounts.wormhole_bridge.fee();
            if fee > 0 {
                solana_program::program::invoke(
                    &solana_program::system_instruction::transfer(
                        &ctx.accounts.owner.key(),
                        &ctx.accounts.wormhole_fee_collector.key(),
                        fee,
                    ),
                    &ctx.accounts.to_account_infos(),
                )?;
            }

            let wormhole_emitter = &ctx.accounts.wormhole_emitter;
            let config = &ctx.accounts.config;


            let mut payload: Vec<u8> = Vec::new();
            SignMessage::serialize(
                &SignMessage::Sign {
                    program_id: *ctx.program_id,
                },
                &mut payload,
            )?;

            wormhole::post_message(
                CpiContext::new_with_signer(
                    ctx.accounts.wormhole_program.to_account_info(),
                    wormhole::PostMessage {
                        config: ctx.accounts.wormhole_bridge.to_account_info(),
                        message: ctx.accounts.wormhole_message.to_account_info(),
                        emitter: wormhole_emitter.to_account_info(),
                        sequence: ctx.accounts.wormhole_sequence.to_account_info(),
                        payer: ctx.accounts.owner.to_account_info(),
                        fee_collector: ctx.accounts.wormhole_fee_collector.to_account_info(),
                        clock: ctx.accounts.clock.to_account_info(),
                        rent: ctx.accounts.rent.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                    },
                    &[
                        &[
                            SEED_PREFIX_SENT,
                            &wormhole::INITIAL_SEQUENCE.to_le_bytes()[..],
                            &[*ctx
                                .bumps
                                .get("wormhole_message")
                                .ok_or(WormholeError::BumpNotFound)?],
                        ],
                        &[wormhole::SEED_PREFIX_EMITTER, &[wormhole_emitter.bump]],
                    ],
                ),
                config.batch_id,
                payload,
                config.finality.try_into().unwrap(),
            )?;
        }

        // Done.
        Ok(())
    }

    pub fn register_emitter(
        ctx: Context<RegisterEmitter>,
        chain: u16,
        address: [u8; 32],
    ) -> Result<()> {
        require!(
            chain > 0 && chain != wormhole::CHAIN_ID_SOLANA && !address.iter().all(|&x| x == 0),
            WormholeError::InvalidForeignEmitter,
        );

        // Save the emitter info into the ForeignEmitter account.
        let emitter = &mut ctx.accounts.foreign_emitter;
        emitter.chain = chain;
        emitter.address = address;

        // Done.
        Ok(())
    }

    pub fn send_message(ctx: Context<SendMessage>, message: Vec<u8>) -> Result<()> {
        let fee = ctx.accounts.wormhole_bridge.fee();
        if fee > 0 {
            solana_program::program::invoke(
                &solana_program::system_instruction::transfer(
                    &ctx.accounts.payer.key(),
                    &ctx.accounts.wormhole_fee_collector.key(),
                    fee,
                ),
                &ctx.accounts.to_account_infos(),
            )?;
        }

        let wormhole_emitter = &ctx.accounts.wormhole_emitter;
        let config = &ctx.accounts.config;

        let payload: Vec<u8> = Sign::SignMessage { message }.try_to_vec()?;

        wormhole::post_message(
            CpiContext::new_with_signer(
                ctx.accounts.wormhole_program.to_account_info(),
                wormhole::PostMessage {
                    config: ctx.accounts.wormhole_bridge.to_account_info(),
                    message: ctx.accounts.wormhole_message.to_account_info(),
                    emitter: wormhole_emitter.to_account_info(),
                    sequence: ctx.accounts.wormhole_sequence.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    fee_collector: ctx.accounts.wormhole_fee_collector.to_account_info(),
                    clock: ctx.accounts.clock.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
                &[
                    &[
                        SEED_PREFIX_SENT,
                        &ctx.accounts.wormhole_sequence.next_value().to_le_bytes()[..],
                        &[*ctx
                            .bumps
                            .get("wormhole_message")
                            .ok_or(WormholeError::BumpNotFound)?],
                    ],
                    &[wormhole::SEED_PREFIX_EMITTER, &[wormhole_emitter.bump]],
                ],
            ),
            config.batch_id,
            payload,
            config.finality.try_into().unwrap(),
        )?;

        // Done.
        Ok(())
    }

    pub fn receive_message(ctx: Context<ReceiveMessage>, vaa_hash: [u8; 32]) -> Result<()> {
        let posted_message = &ctx.accounts.posted;

        if let Sign::SignMessage { message } = posted_message.data() {
            // HelloWorldMessage cannot be larger than the maximum size of the account.
            require!(
                message.len() <= MESSAGE_MAX_LENGTH,
                WormholeError::InvalidMessage,
            );

            // Save batch ID, keccak256 hash and message payload.
            let received = &mut ctx.accounts.received;
            received.batch_id = posted_message.batch_id();
            received.wormhole_message_hash = vaa_hash;
            received.message = message.clone();

            // Done
            Ok(())
        } else {
            Err(WormholeError::InvalidMessage.into())
        }
    }
}