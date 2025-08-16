use omp::{
    core::{
        DisableInteriorEnterExits, EnableStuntBonusForAll, SetGameModeText, SetNameTagDrawDistance,
        SetWeather, SetWorldTime, ShowNameTags, ShowPlayerMarkers,
    },
    events::Events,
    main,
    players::{
        Player, PlayerWeapon, WeaponSlotData,
    },
    register,
    types::{
        colour::Colour,
    },
};


struct GrandLarc {
    colour_white: Colour,
   
}

impl Events for GrandLarc {
    fn on_player_connect(&mut self, player: Player) {
        player.game_text("Larceny", 3000, 4);
		player.send_client_message(
            self.colour_white,
            "hello, welcome to Grand Larceny",
        );
    }

    fn on_player_spawn(&mut self, player: Player) {
        if player.is_npc() {
            return;
        }

        player.set_interior(0);
        player.toggle_clock(false);
        player.reset_money();
        player.give_money(30000);

        player.give_weapon(WeaponSlotData::new(PlayerWeapon::Colt45, 100));
        player.toggle_clock(false);
    }

}

#[main]
pub fn game_entry() -> Result<(), Box<dyn std::error::Error>> {
    SetGameModeText("Grand ก Larceny");
    ShowPlayerMarkers(1);
    ShowNameTags(true);
    SetNameTagDrawDistance(40.0);
    EnableStuntBonusForAll(false);
    DisableInteriorEnterExits();
    SetWeather(2);
    SetWorldTime(11);

    let game = GrandLarc {
        colour_white: Colour::from_rgba(0xFFFFFFFF),
    };

    register!(game);
    Ok(())
}
