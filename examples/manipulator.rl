type {
    Pose
    float
    Accel
}


skillset manipulator{
    resource {
        arm_status {
            state{Hp Motion Static}
            initial Hp
            transition {Hp -> Motion Motion -> Static Static -> Motion Motion -> Hp}
        }

        control_mode {
            state{ManualMode GoalDriven InteractionMode PickandPlace}
            initial  ManualMode
            transition {ManualMode -> GoalDriven
                        GoalDriven -> ManualMode 
                        ManualMode -> InteractionMode 
                        InteractionMode -> ManualMode
                        ManualMode -> PickandPlace
                        PickandPlace -> ManualMode
                        }
        }

    }

    event {
        from_manual_to_goaldriven {
            guard control_mode == ManualMode
            effect control_mode -> GoalDriven
        }

        from_goaldriven_to_manual {
            guard control_mode == GoalDriven
            effect control_mode -> ManualMode
        }

        from_manual_to_interaction {
            guard control_mode == ManualMode
            effect control_mode -> InteractionMode
        }

        from_interaction_to_manual {
            guard control_mode == InteractionMode
            effect control_mode -> ManualMode
        }

        from_manual_to_pick {
            guard control_mode == ManualMode
            effect control_mode -> PickandPlace
        }

        from_pick_to_manual {
            guard control_mode == PickandPlace
            effect control_mode -> ManualMode
        }

    }

    skill arm_moving_manual {

        input target:Accel

        precondition {
            is_static_Hp: arm_status == Hp or arm_status == Static 
            is_angular:  control_mode == ManualMode 
        }

        start {arm_status -> Motion}

        invariant {
            is_angular {
                guard control_mode == ManualMode 
                effect {control_mode -> ManualMode
                        arm_status -> Static
                        }
                }

        }

        progress {
            period 100 ms
            message {
            pos_err: float
            att_err: float
            }
        }

        interrupt  {
            interrupting true
            effect arm_status -> Static
        }

        success is_ready {
            effect arm_status -> Static
        }

        failure {
        singularity_encountered{effect arm_status -> Static}
        stop_arm{effect arm_status -> Static}
        }
    }

    skill arm_moving_goaldriven {

        input target:Pose

        precondition {
            is_static_Hp:  arm_status == Hp or arm_status == Static 
            is_cartesian:  control_mode == GoalDriven 
        }

        start {arm_status -> Motion}

        invariant {
            is_auto {
                guard control_mode == GoalDriven 
                effect {control_mode -> ManualMode 
                        arm_status -> Static
                        }
            }
        }

        progress {
            period 1 ms
            message {
            pos_err: float
            att_err: float
            }
        }

        interrupt  {
            interrupting true
            effect arm_status -> Static
        }

        success is_ready {effect arm_status -> Static}

        failure {
        singularity_encountered{effect arm_status -> Static}
        stop_arm {effect arm_status -> Static}
        }
    }

    skill arm_moving_interaction {

        precondition {
            is_static_Hp:  arm_status == Hp or arm_status == Static 
            is_auto_force: control_mode == InteractionMode 
        }

        start {arm_status -> Motion}

        invariant {
            is_force {
                guard control_mode == InteractionMode
                effect {control_mode -> ManualMode 
                        arm_status -> Static
                        }
            }
        }

        progress {
            period 100 ms
            message {
            pos_err: float
            att_err: float
            }
        }

        interrupt {
            interrupting true
            effect arm_status -> Static
        }

        success is_ready {effect arm_status -> Static}

        failure {
        singularity_encountered{effect arm_status -> Static}
        stop_arm {effect arm_status -> Static}
        }
    }

    skill arm_moving_pickplace {

        precondition {
            is_static_Hp: arm_status == Hp or arm_status == Static
            is_auto_torque: control_mode == PickandPlace 
        }

        start {arm_status -> Motion}

        invariant {
            is_torque {
                guard control_mode == PickandPlace
                effect {control_mode -> ManualMode 
                        arm_status -> Static
                        }
            }
        }

        progress {
            period 100 ms
            message {
            pos_err: float
            att_err: float
            }
        }

        interrupt  {
            interrupting true
            effect arm_status -> Static
        }    


        success is_ready {effect arm_status -> Static}

        failure {
        singularity_encountered{effect arm_status -> Static}
        stop_arm {effect arm_status -> Static}
        }
    }


    skill arm_moving_to_HP {

        precondition {
            is_static: arm_status == Static or arm_status == Hp
            is_any_controller: control_mode == ManualMode or control_mode == GoalDriven or control_mode == InteractionMode or control_mode == PickandPlace 

        }

        start {arm_status -> Motion}

        invariant {
            is_angular {
                guard control_mode == ManualMode 
                effect {control_mode -> ManualMode 
                        arm_status -> Static
                        } 
            }
        }

        progress {
            period 100 ms
            message error: float
        }

        interrupt  {
            interrupting true
            effect arm_status -> Static
        }

        success is_ready {effect arm_status -> Hp}

        failure stop_arm{effect arm_status -> Static}
    }


}
