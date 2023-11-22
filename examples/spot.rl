type {
    Pose2D
    PoseArray
    Float
    GeoPoint
}

skillset spot {

    data {
        battery: Float
        position: GeoPoint
        home: GeoPoint
    }

    resource {
        power_status {
            state { PowerOff PowerOn }
            initial PowerOff
            transition all
        }  
        lease_status {
            state { AutoMode ManualMode }
            initial AutoMode
            transition all
        }
        spot_status {
            state { Sitting Standing }
            initial Sitting
            transition all
        }
        control_mode {
            state { Idle Busy }
            initial Idle
            transition all
        }
        origin_status {
            state { OriginNotSet OriginSet }
            initial OriginNotSet
            transition {OriginNotSet -> OriginSet}
        }
        heading_status {
            state { HeadingNotSet HeadingSet }
            initial HeadingNotSet
            transition {HeadingNotSet -> HeadingSet}
        }
    }

    event {
        toauto_frommanual {
            guard lease_status == ManualMode
            effect lease_status -> AutoMode
        }
        tomanual_fromauto {
            guard lease_status == AutoMode
            effect lease_status -> ManualMode
        }
        power_switchoff {
            guard power_status == PowerOn
            effect {
                power_status -> PowerOff
                spot_status -> Sitting
            }
        }
        power_switchon {
            guard power_status == PowerOff
            effect power_status -> PowerOn
        }
        status_standing {
            guard spot_status == Sitting and power_status == PowerOn
            effect spot_status -> Standing
        }
        status_sitting {
            guard spot_status == Standing
            effect spot_status -> Sitting
        }
    }

    skill init_power {
        precondition {
            is_sitting: spot_status == Sitting
            canmove   : lease_status == AutoMode and control_mode == Idle
            is_powered : power_status == PowerOff
        }
        start {control_mode -> Busy}
        progress{period 1 sec}
        success is_poweredon {
            effect control_mode -> Idle
            postcondition power_status == PowerOn
        }
        failure {
            no_battery {effect control_mode -> Idle}
            is_estopped {effect control_mode -> Idle}
        }
    }

    skill safe_poweroff {
        precondition {
            is_sitting: spot_status == Sitting
            canmove   : lease_status == AutoMode and control_mode == Idle
            is_powered : power_status == PowerOn
        }
        start {control_mode -> Busy}
        progress{period 1 sec}
        success is_poweredoff {
            effect control_mode -> Idle
            postcondition power_status == PowerOff
        }
        failure couldnot_poweroff {effect control_mode -> Idle}
    }

    skill standup {
        precondition {
            is_sitting: spot_status == Sitting
            canmove   : lease_status == AutoMode and control_mode == Idle
            is_powered : power_status == PowerOn
        }
        start {control_mode -> Busy}
        invariant {
            is_powered {
                guard power_status == PowerOn
                effect control_mode -> Idle
            }
        }
        success is_standing {
            effect control_mode -> Idle
            postcondition spot_status == Standing
        }
        failure couldnot_stand {effect control_mode -> Idle}
    }

    skill sitdown {
        precondition {
            is_standing: spot_status == Standing
            canmove    : lease_status == AutoMode and control_mode == Idle
            is_powered  : power_status == PowerOn
        }
        start {control_mode -> Busy}
        invariant {
            is_powered {
                guard power_status == PowerOn
                effect control_mode -> Idle
            }
        }
        success is_sitting {
            effect control_mode -> Idle
            postcondition spot_status == Sitting
        }
        failure couldnot_sit {effect control_mode -> Idle}
    }

    skill go_to_body {
        input target: Pose2D
        output position: Pose2D
        precondition {
            iswalking: spot_status == Standing
            canmove  : lease_status == AutoMode and control_mode == Idle
            is_powered: power_status == PowerOn
        }
        start {control_mode -> Busy}
        invariant {
            is_auto {
                guard lease_status == AutoMode
                effect control_mode -> Idle
            }
            is_powered {
                guard spot_status == Standing
                effect control_mode -> Idle
            }
        }
        progress{
            period 1 sec
            message distance: Float
        }
        interrupt {
            interrupting true
            effect control_mode -> Idle
        }
        success is_arrived {effect control_mode -> Idle}
        failure not_arrived {effect control_mode -> Idle}
    }

    skill go_to_waypoint {
        input target: GeoPoint
        output position: GeoPoint
        precondition {
            iswalking: spot_status == Standing
            canmove  : lease_status == AutoMode and control_mode == Idle
            is_powered: power_status == PowerOn
            hasorigin: origin_status == OriginSet
        }
        start {control_mode -> Busy}
        invariant {
            is_auto {
                guard lease_status == AutoMode
                effect control_mode -> Idle
            }
            is_powered {
                guard spot_status == Standing
                effect control_mode -> Idle
            }
        }
        progress{
            period 1 sec
            message distance: Float
        }
        interrupt {
            interrupting true
            effect control_mode -> Idle
        }
        success is_arrived {effect control_mode -> Idle}
        failure not_arrived {effect control_mode -> Idle}
    }

    skill go_to_odom {
        input target: Pose
        output pose: Pose
        precondition {
            iswalking: spot_status == Standing
            canmove  : lease_status == AutoMode and control_mode == Idle
            is_powered: power_status == PowerOn
        }
        start {control_mode -> Busy}
        invariant {
            is_auto {
                guard lease_status == AutoMode
                effect control_mode -> Idle
            }
            is_powered {
                guard spot_status == Standing
                effect control_mode -> Idle
            }
        }
        progress{
            period 1 sec
            message distance: Float
        }
        interrupt {
            interrupting true
            effect control_mode -> Idle
        }
        success is_arrived {effect control_mode -> Idle}
        failure not_arrived {effect control_mode -> Idle}
    }

    skill capture_origin {
        success is_set {effect origin_status -> OriginSet}
        failure is_not_set {}
    }

    skill set_heading {
        output heading: Float
        precondition {
            iswalking: spot_status == Standing
            canmove  : lease_status == AutoMode and control_mode == Idle
            is_powered: power_status == PowerOn
        }
        start {control_mode -> Busy}
        invariant {
            is_auto {
                guard lease_status == AutoMode
                effect control_mode -> Idle
            }
            is_powered {
                guard spot_status == Standing
                effect control_mode -> Idle
            }
        }
        progress{
            period 1 sec
            message remaining_time: Float
        }
        interrupt {
            interrupting true
            effect control_mode -> Idle
        }
        success heading_isset {
            effect {
                control_mode -> Idle
                heading_status -> HeadingSet
            }
        }
        failure couldnot_setheading {effect control_mode -> Idle}
    }

    skill go_back_on_track {
        output position: GeoPoint
        precondition {
            iswalking: spot_status == Standing
            canmove  : lease_status == AutoMode and control_mode == Idle
            is_powered: power_status == PowerOn
        }
        start {control_mode -> Busy}
        invariant {
            is_auto {
                guard lease_status == AutoMode
                effect control_mode -> Idle
            }
            is_powered {
                guard spot_status == Standing
                effect control_mode -> Idle
            }
        }
        progress{
            period 1 sec
            message distance: Float
        }
        interrupt {
            interrupting true
            effect control_mode -> Idle
        }
        success is_arrived {effect control_mode -> Idle}
        failure not_arrived {effect control_mode -> Idle}
    }

    skill teleop {
        precondition {
            iswalking: spot_status == Standing
            canmove  : lease_status == AutoMode and control_mode == Idle
            is_powered: power_status == PowerOn
        }
        start {
            control_mode -> Busy
            lease_status -> ManualMode
        }
        invariant {
            is_auto {
                guard lease_status == ManualMode
                effect control_mode -> Idle
            }
            is_powered {
                guard spot_status == Standing
                effect control_mode -> Idle
            }
        }
        progress{
            period 1 sec
            message ping: Float
        }
        interrupt { 
            interrupting true
            effect {
                control_mode -> Idle
                lease_status -> AutoMode
            }
        }
        success teleop_done {
            effect {
                control_mode -> Idle
                lease_status -> AutoMode
            }
        }
        failure teleop_failed {
            effect {
                control_mode -> Idle
                lease_status -> AutoMode
            }
        }
    }

}
