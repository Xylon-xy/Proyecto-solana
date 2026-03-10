use anchor_lang::prelude::*;  ///

declare_id!("72FMHp1eFExSYvGjtZtbbJoLN4fbuZk2HiwvketQP5yt");//nombre

#[program]//libreria  programa 
pub mod veterinaria_program {// buautiso
    // <--- Aquí empieza el Módulo
    use super::*;// libreria

    // --- CREATE: Registrar una mascota ---
    //funcion
    pub fn registrar_mascota(
        ctx: Context<RegisterPet>, //decalracion matriculaa
        nombre: String, // varibles 
        especie: String,
        edad: u8,// tamaño  numero
    ) -> Result<()> {
        let mascota = &mut ctx.accounts.pet_account;
        mascota.owner = *ctx.accounts.owner.key;//nombre dueños
        mascota.nombre = nombre;
        mascota.especie = especie;
        mascota.edad = edad;
        mascota.historial = "Primera consulta: Saludable".to_string();

        msg!("Mascota {} registrada con éxito!", mascota.nombre);
        Ok(())
    }

    // --- UPDATE: Actualizar historial médico ---
    pub fn actualizar_historial(
        ctx: Context<UpdatePet>,
        _nombre: String,
        nuevo_historial: String,
    ) -> Result<()> {
        let mascota = &mut ctx.accounts.pet_account;
        mascota.historial = nuevo_historial;

        msg!("Historial de {} actualizado.", mascota.nombre);
        Ok(())
    }

    // --- DELETE: Dar de alta (Cerrar cuenta) ---
    pub fn dar_de_alta(_ctx: Context<DischargePet>, nombre: String) -> Result<()> {
        msg!(
            "La mascota {} ha sido dada de alta. Cuenta cerrada.",
            nombre
        );
        Ok(())
    }
} // <--- Aquí termina el Módulo

// --- 2. EL ESTADO (Estructura de la cuenta) ---
#[account]
#[derive(InitSpace)]
pub struct PetAccount {
    pub owner: Pubkey,
    #[max_len(32)]
    pub nombre: String,
    #[max_len(32)]
    pub especie: String,
    pub edad: u8,
    #[max_len(150)]
    pub historial: String,
}

// --- 3. VALIDACIÓN DE CUENTAS (Contextos) ---

#[derive(Accounts)]
#[instruction(nombre: String)] // Pasamos el nombre para generar la semilla (PDA)
pub struct RegisterPet<'info> {
    #[account(
        init,
        seeds = [ nombre.as_bytes(),owner.key().as_ref()], 
        //nombre.as_bytes()
        bump, 
        payer = owner, 
        space = 8 + PetAccount::INIT_SPACE
    )]
    pub pet_account: Account<'info, PetAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct UpdatePet<'info> {
    #[account(
        mut,
        seeds = [nombre.as_bytes(), owner.key().as_ref()],
        bump,
        has_one = owner // Solo el dueño puede editar
    )]
    pub pet_account: Account<'info, PetAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(nombre: String)]
pub struct DischargePet<'info> {
    #[account(
        mut,
        seeds = [nombre.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner // Borra la cuenta y regresa el SOL al dueño
    )]
    pub pet_account: Account<'info, PetAccount>,
    pub owner: Signer<'info>,
}


