﻿import { ModuleWithProviders }  from '@angular/core';
import { Routes, RouterModule } from '@angular/router';
import { homeRoutes } from './pages/home/home.routes';
import { gameRoutes } from './pages/game/game.routes';

export const routes: Routes = [
    ...homeRoutes,
    ...gameRoutes
  ];

  export const routing: ModuleWithProviders = RouterModule.forRoot(routes);
